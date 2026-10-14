---
date created: 2025-03-24
date modified: 2025-10-29
uid: 4e527a8d-9fac-4f78-aaf4-15c7b71e501f
---
# Summary

本质是[Transformers](Transformers.md)这种通用推理框架是针对所有[深度学习@](深度学习@.md)任务的，而 vllm 针对[LLM大模型](LLM大模型.md)这种特殊任务做了特殊优化，所以更快。
1. 自动批处理
2. [GPU Kernel 内核](GPU%20Kernel%20内核.md)
3. [PagedAttention](PagedAttention.md) 分页注意力

# Cues

你这个是**“为什么后端从 FlashAttention-2 降级到 xFormers 的经典问题变体”**。

简明答案：**因为你用的是 T4（Turing，SM_75）**。
FlashAttention-2 需要 **Ampere/Hopper 级别的 GPU（≥SM_80，如 A100/A10/L4/H100）** 才能启用其关键指令路径（如 `cp.async` 等）。在 **Volta(V100, SM_70)** 和 **Turing(T4, SM_75)** 上不满足硬件要求，vLLM 就会自动回退到 **xFormers** 后端，所以日志里写着：
# Notes

## 部署参数

**`gpu-memory-utilization` 会影响推理速度，但影响程度取决于使用场景：**

1. **并发处理能力**
   - 较高（0.9）：能同时处理更多请求，吞吐量更高
   - 较低（0.5）：并发请求数受限，吞吐量可能降低

2. **KV Cache 大小**
   - 较高：可缓存更多 KV，减少重复计算，提升长文本生成效率
   - 较低：KV Cache 受限，长文本或长上下文可能受影响

3. **单请求延迟**
   - 对单个请求的延迟影响通常较小，主要影响并发吞吐

## 针对你的情况（Qwen3-4B 模型）

- 4B 参数量较小，0.5 利用率通常足够
- 单请求延迟通常无明显影响
- 并发处理能力可能略降，但仍能支持一定并发
- 如果是长文本生成或高并发，建议适当提高

## 建议

- 如果只是单请求或少并发：`0.5` 通常足够，速度影响不明显
- 如果需要高并发或长文本：尽量提高（如 `0.7-0.8`），前提是 GPU 显存允许

**总结**：0.5 不会显著影响单请求速度；在高并发或长文本场景下，适当提高利用率能提升整体吞吐。



## 部署脚本参考
```python
#!/usr/bin/env python3
"""
vLLM 本地模型部署脚本
功能：启动OpenAI兼容服务器 + 测试
适用于已下载到本地的模型
"""

import os
import sys
import time
import json
import subprocess
import argparse
from pathlib import Path
from typing import Optional

# ============= 配置类 =============
class Config:
    """配置管理"""
    def __init__(self, verbose: bool = False):
        self.project_dir = Path.home() / "qwen-vllm-server"
        # 自动查找 qwen3-4b 目录
        possible_paths = [
            Path.cwd() / "qwen3-4b",  # 当前目录下
            Path.home() / "qwen3-4b",  # home目录下
            Path("/home/yishou.liu/model/qwen3-4b/qwen3-4b"),  # 完整路径
        ]
        self.model_path = None
        if verbose:
            print_info("正在查找模型路径...")
        for path in possible_paths:
            if verbose:
                print_info(f"  尝试: {path} -> 存在: {path.exists()}")
            if path.exists() and (path / "config.json").exists():
                self.model_path = str(path.resolve())  # 使用绝对路径
                if verbose:
                    print_success(f"  找到模型: {self.model_path}")
                break
        if self.model_path is None:
            self.model_path = str(Path.cwd() / "qwen3-4b")  # 默认使用当前目录
            if verbose:
                print_warning(f"  未找到模型，使用默认路径: {self.model_path}")
        self.model_name = "Qwen3-4B"  # 服务中显示的模型名
        self.port = 8000
        self.host = "0.0.0.0"
        self.gpu_ids = "0"  # 使用的GPU，多卡用逗号分隔 "0,1,2,3"
        self.tensor_parallel_size = 1  # 张量并行度
        self.gpu_memory_utilization = 0.9
        self.max_model_len = 4096
        
    def save(self, filepath: Path):
        """保存配置到文件"""
        config_dict = {
            'model_path': self.model_path,
            'model_name': self.model_name,
            'port': self.port,
            'host': self.host,
            'gpu_ids': self.gpu_ids,
            'tensor_parallel_size': self.tensor_parallel_size,
            'gpu_memory_utilization': self.gpu_memory_utilization,
            'max_model_len': self.max_model_len,
        }
        with open(filepath, 'w', encoding='utf-8') as f:
            json.dump(config_dict, f, indent=2, ensure_ascii=False)
        print(f"✅ 配置已保存: {filepath}")
    
    @classmethod
    def load(cls, filepath: Path):
        """从文件加载配置"""
        with open(filepath, 'r', encoding='utf-8') as f:
            config_dict = json.load(f)
        config = cls()
        config.model_path = config_dict.get('model_path', config.model_path)
        config.model_name = config_dict.get('model_name', config.model_name)
        config.port = config_dict.get('port', config.port)
        config.host = config_dict.get('host', config.host)
        config.gpu_ids = config_dict.get('gpu_ids', config.gpu_ids)
        config.tensor_parallel_size = config_dict.get('tensor_parallel_size', config.tensor_parallel_size)
        config.gpu_memory_utilization = config_dict.get('gpu_memory_utilization', config.gpu_memory_utilization)
        config.max_model_len = config_dict.get('max_model_len', config.max_model_len)
        return config


# ============= 工具函数 =============
def print_header(text: str):
    """打印标题"""
    print(f"\n{'='*70}")
    print(f"  {text}")
    print(f"{'='*70}")


def print_info(text: str):
    """打印信息"""
    print(f"ℹ️  {text}")


def print_success(text: str):
    """打印成功信息"""
    print(f"✅ {text}")


def print_error(text: str):
    """打印错误信息"""
    print(f"❌ {text}")


def print_warning(text: str):
    """打印警告信息"""
    print(f"⚠️  {text}")


# ============= 模型验证 =============
def validate_model_path(model_path: str, verbose: bool = True) -> bool:
    """验证模型路径是否有效"""
    path = Path(model_path)
    
    if verbose:
        print_info(f"正在验证模型路径: {path}")
        print_info(f"路径是否存在: {path.exists()}")
    
    if not path.exists():
        print_error(f"模型路径不存在: {model_path}")
        return False
    
    # 检查必要的模型文件
    required_files = ['config.json']  # 只检查最基本的文件
    safetensors_files = list(path.glob('*.safetensors'))
    
    if verbose:
        print_info(f"目录内容: {list(path.iterdir())[:5]}...")  # 显示前5个文件
    
    for req_file in required_files:
        file_path = path / req_file
        if verbose:
            print_info(f"检查文件 {req_file}: {file_path.exists()}")
        if not file_path.exists():
            print_error(f"缺少必要文件: {req_file}")
            return False
    
    if not safetensors_files:
        print_error("未找到 .safetensors 模型文件")
        return False
    
    print_success(f"模型验证通过: {model_path}")
    print_info(f"找到 {len(safetensors_files)} 个模型分片文件")
    return True


# ============= 服务器管理 =============
class VLLMServer:
    """vLLM服务器管理"""
    
    def __init__(self, config: Config):
        self.config = config
    
    def start(self):
        """启动vLLM服务器"""
        print_header("启动 vLLM OpenAI 兼容服务器")
        
        # 验证模型路径
        if not validate_model_path(self.config.model_path):
            print_error("模型验证失败，请检查模型路径")
            sys.exit(1)
        
        print_info("服务器配置:")
        print(f"  模型路径: {self.config.model_path}")
        print(f"  模型名称: {self.config.model_name}")
        print(f"  服务地址: http://{self.config.host}:{self.config.port}")
        print(f"  GPU: {self.config.gpu_ids}")
        print(f"  并行度: {self.config.tensor_parallel_size}")
        print(f"  显存利用率: {self.config.gpu_memory_utilization}")
        print(f"  最大上下文: {self.config.max_model_len}")
        
        # 设置环境变量
        os.environ['CUDA_VISIBLE_DEVICES'] = self.config.gpu_ids
        
        # 构建启动命令
        cmd = [
            sys.executable, "-m", "vllm.entrypoints.openai.api_server",
            "--model", self.config.model_path,
            "--host", self.config.host,
            "--port", str(self.config.port),
            "--trust-remote-code",
            "--tensor-parallel-size", str(self.config.tensor_parallel_size),
            "--gpu-memory-utilization", str(self.config.gpu_memory_utilization),
            "--max-model-len", str(self.config.max_model_len),
            "--dtype", "auto",
            "--served-model-name", self.config.model_name,
        ]
        
        print_info("\n启动命令:")
        print(" ".join(cmd))
        print("\n" + "="*70)
        print_warning("服务器启动中，按 Ctrl+C 停止服务")
        print("="*70 + "\n")
        
        try:
            # 启动服务器
            subprocess.run(cmd, check=True)
        except KeyboardInterrupt:
            print_info("\n收到停止信号，正在关闭服务器...")
        except Exception as e:
            print_error(f"服务器启动失败: {e}")
            sys.exit(1)


# ============= 测试客户端 =============
class TestClient:
    """测试客户端"""
    
    def __init__(self, config: Config):
        self.config = config
        self.base_url = f"http://localhost:{config.port}/v1"
    
    def test_all(self):
        """运行所有测试"""
        print_header("测试服务器")
        
        if not self._wait_for_server():
            print_error("服务器未响应，请先启动服务器")
            return False
        
        success = True
        success = self._test_connection() and success
        success = self._test_chat_completion() and success
        success = self._test_streaming() and success
        success = self._test_batch() and success
        
        if success:
            print_success("\n所有测试通过！🎉")
        else:
            print_warning("\n部分测试失败")
        
        return success
    
    def _wait_for_server(self, timeout: int = 60):
        """等待服务器启动"""
        print_info(f"等待服务器启动（超时{timeout}秒）...")
        
        import requests
        start_time = time.time()
        
        while time.time() - start_time < timeout:
            try:
                response = requests.get(f"http://localhost:{self.config.port}/health", timeout=2)
                if response.status_code == 200:
                    print_success("服务器已就绪")
                    return True
            except:
                pass
            time.sleep(2)
        
        return False
    
    def _test_connection(self):
        """测试连接"""
        print("\n" + "-"*70)
        print("测试1: 连接服务器")
        print("-"*70)
        
        try:
            from openai import OpenAI
            
            client = OpenAI(base_url=self.base_url, api_key="EMPTY")
            models = client.models.list()
            
            print_success("连接成功")
            print(f"可用模型: {[m.id for m in models.data]}")
            return True
            
        except Exception as e:
            print_error(f"连接失败: {e}")
            return False
    
    def _test_chat_completion(self):
        """测试对话补全"""
        print("\n" + "-"*70)
        print("测试2: 对话补全")
        print("-"*70)
        
        try:
            from openai import OpenAI
            
            client = OpenAI(base_url=self.base_url, api_key="EMPTY")
            
            print_info("发送请求: 用一句话介绍你自己")
            
            response = client.chat.completions.create(
                model=self.config.model_name,
                messages=[
                    {"role": "system", "content": "你是一个有帮助的AI助手"},
                    {"role": "user", "content": "用一句话介绍你自己"}
                ],
                temperature=0.7,
                max_tokens=100,
            )
            
            print_success("对话成功")
            print(f"回答: {response.choices[0].message.content}")
            print(f"Token使用: prompt={response.usage.prompt_tokens}, "
                  f"completion={response.usage.completion_tokens}, "
                  f"total={response.usage.total_tokens}")
            return True
            
        except Exception as e:
            print_error(f"对话失败: {e}")
            return False
    
    def _test_streaming(self):
        """测试流式输出"""
        print("\n" + "-"*70)
        print("测试3: 流式输出")
        print("-"*70)
        
        try:
            from openai import OpenAI
            
            client = OpenAI(base_url=self.base_url, api_key="EMPTY")
            
            print_info("发送请求: 从1数到5")
            print("AI: ", end="", flush=True)
            
            stream = client.chat.completions.create(
                model=self.config.model_name,
                messages=[{"role": "user", "content": "从1数到5，每个数字用逗号分隔"}],
                temperature=0.7,
                max_tokens=50,
                stream=True,
            )
            
            for chunk in stream:
                if chunk.choices[0].delta.content:
                    print(chunk.choices[0].delta.content, end="", flush=True)
            
            print("\n" + "✅ 流式输出成功")
            return True
            
        except Exception as e:
            print(f"\n❌ 流式输出失败: {e}")
            return False
    
    def _test_batch(self):
        """测试批量推理"""
        print("\n" + "-"*70)
        print("测试4: 批量推理")
        print("-"*70)
        
        try:
            from openai import OpenAI
            
            client = OpenAI(base_url=self.base_url, api_key="EMPTY")
            
            questions = [
                "1+1等于几？",
                "Python是什么？",
                "什么是AI？",
            ]
            
            print_info(f"批量发送 {len(questions)} 个请求")
            start_time = time.time()
            
            for i, question in enumerate(questions, 1):
                response = client.chat.completions.create(
                    model=self.config.model_name,
                    messages=[{"role": "user", "content": question}],
                    temperature=0.7,
                    max_tokens=50,
                )
                print(f"  {i}. Q: {question}")
                print(f"     A: {response.choices[0].message.content[:50]}...")
            
            elapsed = time.time() - start_time
            print_success(f"批量推理完成，耗时: {elapsed:.2f}秒")
            return True
            
        except Exception as e:
            print_error(f"批量推理失败: {e}")
            return False


# ============= 命令行接口 =============
def create_parser():
    """创建命令行参数解析器"""
    parser = argparse.ArgumentParser(
        description="vLLM 本地模型部署脚本",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
示例:
  # 启动服务器（使用默认配置）
  python %(prog)s serve
  
  # 测试服务器
  python %(prog)s test
  
  # 自定义配置启动
  python %(prog)s serve --model-path ~/qwen3-4b --port 8001 --gpu-ids 0,1
  
  # 后台启动
  nohup python %(prog)s serve > server.log 2>&1 &
        """
    )
    
    parser.add_argument(
        'command',
        choices=['serve', 'test', 'config'],
        help='执行的命令'
    )
    
    parser.add_argument('--model-path', default=None,
                        help='本地模型路径 (默认: 自动查找 qwen3-4b)')
    parser.add_argument('--model-name', default='Qwen3-4B',
                        help='服务中显示的模型名称 (默认: Qwen3-4B)')
    parser.add_argument('--port', type=int, default=8000,
                        help='服务器端口 (默认: 8000)')
    parser.add_argument('--host', default='0.0.0.0',
                        help='服务器地址 (默认: 0.0.0.0)')
    parser.add_argument('--gpu-ids', default='0',
                        help='使用的GPU ID，多卡用逗号分隔 (默认: 0)')
    parser.add_argument('--tensor-parallel-size', type=int, default=1,
                        help='张量并行度 (默认: 1)')
    parser.add_argument('--gpu-memory-utilization', type=float, default=0.9,
                        help='GPU显存利用率 (默认: 0.9)')
    parser.add_argument('--max-model-len', type=int, default=4096,
                        help='最大上下文长度 (默认: 4096)')
    
    return parser


# ============= 主函数 =============
def main():
    """主函数"""
    parser = create_parser()
    args = parser.parse_args()
    
    # 创建配置
    config = Config(verbose=True)  # 启用调试信息
    if args.model_path:
        config.model_path = args.model_path
    config.model_name = args.model_name
    config.port = args.port
    config.host = args.host
    config.gpu_ids = args.gpu_ids
    config.tensor_parallel_size = args.tensor_parallel_size
    config.gpu_memory_utilization = args.gpu_memory_utilization
    config.max_model_len = args.max_model_len
    
    # 创建项目目录
    config.project_dir.mkdir(parents=True, exist_ok=True)
    config_file = config.project_dir / "config.json"
    
    # 执行命令
    if args.command == 'config':
        # 保存配置
        config.save(config_file)
        print_success("配置文件已创建")
        print_info(f"位置: {config_file}")
        
    elif args.command == 'serve':
        # 启动服务器
        config.save(config_file)
        server = VLLMServer(config)
        server.start()
        
    elif args.command == 'test':
        # 测试服务器
        if config_file.exists():
            config = Config.load(config_file)
        
        tester = TestClient(config)
        success = tester.test_all()
        sys.exit(0 if success else 1)


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print_info("\n用户中断")
        sys.exit(0)
    except Exception as e:
        print_error(f"程序错误: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)
```








```python
#!/usr/bin/env python3
"""
vLLM + 魔搭 完整部署脚本
功能：下载模型 + 启动OpenAI兼容服务器 + 测试
"""

import os
import sys
import time
import json
import subprocess
import argparse
from pathlib import Path
from typing import Optional

# ============= 配置类 =============
class Config:
    """配置管理"""
    def __init__(self):
        self.project_dir = Path.home() / "qwen-vllm-server"
        self.model_name = "Qwen/Qwen3-4B"
        self.model_cache_dir = self.project_dir / "models"
        self.port = 8000
        self.host = "0.0.0.0"
        self.gpu_ids = "0"  # 使用的GPU，多卡用逗号分隔 "0,1,2,3"
        self.tensor_parallel_size = 1  # 张量并行度
        self.gpu_memory_utilization = 0.9
        self.max_model_len = 4096
        
    def save(self, filepath: Path):
        """保存配置到文件"""
        config_dict = {
            'model_name': self.model_name,
            'model_cache_dir': str(self.model_cache_dir),
            'port': self.port,
            'host': self.host,
            'gpu_ids': self.gpu_ids,
            'tensor_parallel_size': self.tensor_parallel_size,
            'gpu_memory_utilization': self.gpu_memory_utilization,
            'max_model_len': self.max_model_len,
        }
        with open(filepath, 'w', encoding='utf-8') as f:
            json.dump(config_dict, f, indent=2, ensure_ascii=False)
        print(f"✅ 配置已保存: {filepath}")
    
    @classmethod
    def load(cls, filepath: Path):
        """从文件加载配置"""
        with open(filepath, 'r', encoding='utf-8') as f:
            config_dict = json.load(f)
        config = cls()
        config.model_name = config_dict.get('model_name', config.model_name)
        config.model_cache_dir = Path(config_dict.get('model_cache_dir', config.model_cache_dir))
        config.port = config_dict.get('port', config.port)
        config.host = config_dict.get('host', config.host)
        config.gpu_ids = config_dict.get('gpu_ids', config.gpu_ids)
        config.tensor_parallel_size = config_dict.get('tensor_parallel_size', config.tensor_parallel_size)
        config.gpu_memory_utilization = config_dict.get('gpu_memory_utilization', config.gpu_memory_utilization)
        config.max_model_len = config_dict.get('max_model_len', config.max_model_len)
        return config


# ============= 工具函数 =============
def print_header(text: str):
    """打印标题"""
    print(f"\n{'='*70}")
    print(f"  {text}")
    print(f"{'='*70}")


def print_info(text: str):
    """打印信息"""
    print(f"ℹ️  {text}")


def print_success(text: str):
    """打印成功信息"""
    print(f"✅ {text}")


def print_error(text: str):
    """打印错误信息"""
    print(f"❌ {text}")


def print_warning(text: str):
    """打印警告信息"""
    print(f"⚠️  {text}")


# ============= 模型下载 =============
class ModelDownloader:
    """模型下载器"""
    
    def __init__(self, config: Config):
        self.config = config
        self.model_path_file = config.project_dir / "model_path.txt"
    
    def download(self) -> str:
        """从魔搭下载模型"""
        print_header("步骤1: 下载模型")
        
        # 创建缓存目录
        self.config.model_cache_dir.mkdir(parents=True, exist_ok=True)
        
        # 检查是否已下载
        if self.model_path_file.exists():
            model_path = self.model_path_file.read_text().strip()
            if Path(model_path).exists():
                print_info(f"模型已存在: {model_path}")
                return model_path
        
        print_info(f"开始下载模型: {self.config.model_name}")
        print_info(f"缓存目录: {self.config.model_cache_dir}")
        print_warning("首次下载可能需要较长时间，请耐心等待...")
        
        try:
            from modelscope import snapshot_download
            
            start_time = time.time()
            model_path = snapshot_download(
                self.config.model_name,
                cache_dir=str(self.config.model_cache_dir),
            )
            elapsed = time.time() - start_time
            
            # 保存模型路径
            self.model_path_file.write_text(model_path)
            
            print_success(f"模型下载完成！耗时: {elapsed:.1f}秒")
            print_success(f"模型路径: {model_path}")
            return model_path
            
        except Exception as e:
            print_error(f"模型下载失败: {e}")
            sys.exit(1)
    
    def get_model_path(self) -> Optional[str]:
        """获取已下载的模型路径"""
        if self.model_path_file.exists():
            return self.model_path_file.read_text().strip()
        return None


# ============= 服务器管理 =============
class VLLMServer:
    """vLLM服务器管理"""
    
    def __init__(self, config: Config, model_path: str):
        self.config = config
        self.model_path = model_path
    
    def start(self):
        """启动vLLM服务器"""
        print_header("步骤2: 启动vLLM OpenAI兼容服务器")
        
        print_info("服务器配置:")
        print(f"  模型: {self.model_path}")
        print(f"  地址: http://{self.config.host}:{self.config.port}")
        print(f"  GPU: {self.config.gpu_ids}")
        print(f"  并行度: {self.config.tensor_parallel_size}")
        print(f"  显存利用率: {self.config.gpu_memory_utilization}")
        print(f"  最大上下文: {self.config.max_model_len}")
        
        # 设置环境变量
        os.environ['CUDA_VISIBLE_DEVICES'] = self.config.gpu_ids
        
        # 构建启动命令
        cmd = [
            sys.executable, "-m", "vllm.entrypoints.openai.api_server",
            "--model", self.model_path,
            "--host", self.config.host,
            "--port", str(self.config.port),
            "--trust-remote-code",
            "--tensor-parallel-size", str(self.config.tensor_parallel_size),
            "--gpu-memory-utilization", str(self.config.gpu_memory_utilization),
            "--max-model-len", str(self.config.max_model_len),
            "--dtype", "auto",
            "--served-model-name", self.config.model_name,
        ]
        
        print_info("\n启动命令:")
        print(" ".join(cmd))
        print("\n" + "="*70)
        print_warning("服务器启动中，按 Ctrl+C 停止服务")
        print("="*70 + "\n")
        
        try:
            # 启动服务器
            subprocess.run(cmd, check=True)
        except KeyboardInterrupt:
            print_info("\n收到停止信号，正在关闭服务器...")
        except Exception as e:
            print_error(f"服务器启动失败: {e}")
            sys.exit(1)


# ============= 测试客户端 =============
class TestClient:
    """测试客户端"""
    
    def __init__(self, config: Config):
        self.config = config
        self.base_url = f"http://localhost:{config.port}/v1"
    
    def test_all(self):
        """运行所有测试"""
        print_header("步骤3: 测试服务器")
        
        if not self._wait_for_server():
            print_error("服务器未响应，请先启动服务器")
            return False
        
        success = True
        success = self._test_connection() and success
        success = self._test_chat_completion() and success
        success = self._test_streaming() and success
        success = self._test_batch() and success
        
        if success:
            print_success("\n所有测试通过！🎉")
        else:
            print_warning("\n部分测试失败")
        
        return success
    
    def _wait_for_server(self, timeout: int = 60):
        """等待服务器启动"""
        print_info(f"等待服务器启动（超时{timeout}秒）...")
        
        import requests
        start_time = time.time()
        
        while time.time() - start_time < timeout:
            try:
                response = requests.get(f"http://localhost:{self.config.port}/health", timeout=2)
                if response.status_code == 200:
                    print_success("服务器已就绪")
                    return True
            except:
                pass
            time.sleep(2)
        
        return False
    
    def _test_connection(self):
        """测试连接"""
        print("\n" + "-"*70)
        print("测试1: 连接服务器")
        print("-"*70)
        
        try:
            from openai import OpenAI
            
            client = OpenAI(base_url=self.base_url, api_key="EMPTY")
            models = client.models.list()
            
            print_success("连接成功")
            print(f"可用模型: {[m.id for m in models.data]}")
            return True
            
        except Exception as e:
            print_error(f"连接失败: {e}")
            return False
    
    def _test_chat_completion(self):
        """测试对话补全"""
        print("\n" + "-"*70)
        print("测试2: 对话补全")
        print("-"*70)
        
        try:
            from openai import OpenAI
            
            client = OpenAI(base_url=self.base_url, api_key="EMPTY")
            
            print_info("发送请求: 用一句话介绍你自己")
            
            response = client.chat.completions.create(
                model=self.config.model_name,
                messages=[
                    {"role": "system", "content": "你是一个有帮助的AI助手"},
                    {"role": "user", "content": "用一句话介绍你自己"}
                ],
                temperature=0.7,
                max_tokens=100,
            )
            
            print_success("对话成功")
            print(f"回答: {response.choices[0].message.content}")
            print(f"Token使用: prompt={response.usage.prompt_tokens}, "
                  f"completion={response.usage.completion_tokens}, "
                  f"total={response.usage.total_tokens}")
            return True
            
        except Exception as e:
            print_error(f"对话失败: {e}")
            return False
    
    def _test_streaming(self):
        """测试流式输出"""
        print("\n" + "-"*70)
        print("测试3: 流式输出")
        print("-"*70)
        
        try:
            from openai import OpenAI
            
            client = OpenAI(base_url=self.base_url, api_key="EMPTY")
            
            print_info("发送请求: 从1数到5")
            print("AI: ", end="", flush=True)
            
            stream = client.chat.completions.create(
                model=self.config.model_name,
                messages=[{"role": "user", "content": "从1数到5，每个数字用逗号分隔"}],
                temperature=0.7,
                max_tokens=50,
                stream=True,
            )
            
            for chunk in stream:
                if chunk.choices[0].delta.content:
                    print(chunk.choices[0].delta.content, end="", flush=True)
            
            print("\n" + "✅ 流式输出成功")
            return True
            
        except Exception as e:
            print(f"\n❌ 流式输出失败: {e}")
            return False
    
    def _test_batch(self):
        """测试批量推理"""
        print("\n" + "-"*70)
        print("测试4: 批量推理")
        print("-"*70)
        
        try:
            from openai import OpenAI
            
            client = OpenAI(base_url=self.base_url, api_key="EMPTY")
            
            questions = [
                "1+1等于几？",
                "Python是什么？",
                "什么是AI？",
            ]
            
            print_info(f"批量发送 {len(questions)} 个请求")
            start_time = time.time()
            
            for i, question in enumerate(questions, 1):
                response = client.chat.completions.create(
                    model=self.config.model_name,
                    messages=[{"role": "user", "content": question}],
                    temperature=0.7,
                    max_tokens=50,
                )
                print(f"  {i}. Q: {question}")
                print(f"     A: {response.choices[0].message.content[:50]}...")
            
            elapsed = time.time() - start_time
            print_success(f"批量推理完成，耗时: {elapsed:.2f}秒")
            return True
            
        except Exception as e:
            print_error(f"批量推理失败: {e}")
            return False


# ============= 命令行接口 =============
def create_parser():
    """创建命令行参数解析器"""
    parser = argparse.ArgumentParser(
        description="vLLM + 魔搭 一键部署脚本",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
示例:
  # 下载模型
  python %(prog)s download
  
  # 启动服务器
  python %(prog)s serve
  
  # 测试服务器
  python %(prog)s test
  
  # 一键部署（下载+启动）
  python %(prog)s deploy
  
  # 自定义配置
  python %(prog)s serve --port 8001 --gpu-ids 0,1 --tensor-parallel-size 2
        """
    )
    
    parser.add_argument(
        'command',
        choices=['download', 'serve', 'test', 'deploy', 'config'],
        help='执行的命令'
    )
    
    parser.add_argument('--model-name', default='Qwen/Qwen3-4B',
                        help='模型名称 (默认: Qwen/Qwen3-4B)')
    parser.add_argument('--port', type=int, default=8000,
                        help='服务器端口 (默认: 8000)')
    parser.add_argument('--host', default='0.0.0.0',
                        help='服务器地址 (默认: 0.0.0.0)')
    parser.add_argument('--gpu-ids', default='0',
                        help='使用的GPU ID，多卡用逗号分隔 (默认: 0)')
    parser.add_argument('--tensor-parallel-size', type=int, default=1,
                        help='张量并行度 (默认: 1)')
    parser.add_argument('--gpu-memory-utilization', type=float, default=0.9,
                        help='GPU显存利用率 (默认: 0.9)')
    parser.add_argument('--max-model-len', type=int, default=4096,
                        help='最大上下文长度 (默认: 4096)')
    
    return parser


# ============= 主函数 =============
def main():
    """主函数"""
    parser = create_parser()
    args = parser.parse_args()
    
    # 创建配置
    config = Config()
    config.model_name = args.model_name
    config.port = args.port
    config.host = args.host
    config.gpu_ids = args.gpu_ids
    config.tensor_parallel_size = args.tensor_parallel_size
    config.gpu_memory_utilization = args.gpu_memory_utilization
    config.max_model_len = args.max_model_len
    
    # 创建项目目录
    config.project_dir.mkdir(parents=True, exist_ok=True)
    config_file = config.project_dir / "config.json"
    
    # 执行命令
    if args.command == 'config':
        # 保存配置
        config.save(config_file)
        print_success("配置文件已创建")
        print_info(f"位置: {config_file}")
        
    elif args.command == 'download':
        # 只下载模型
        downloader = ModelDownloader(config)
        downloader.download()
        config.save(config_file)
        
    elif args.command == 'serve':
        # 启动服务器
        downloader = ModelDownloader(config)
        model_path = downloader.get_model_path()
        
        if model_path is None:
            print_warning("模型未下载，开始下载...")
            model_path = downloader.download()
        
        config.save(config_file)
        server = VLLMServer(config, model_path)
        server.start()
        
    elif args.command == 'test':
        # 测试服务器
        if config_file.exists():
            config = Config.load(config_file)
        
        tester = TestClient(config)
        success = tester.test_all()
        sys.exit(0 if success else 1)
        
    elif args.command == 'deploy':
        # 一键部署
        print_header("vLLM + 魔搭 一键部署")
        
        # 1. 下载模型
        downloader = ModelDownloader(config)
        model_path = downloader.download()
        
        # 2. 保存配置
        config.save(config_file)
        
        # 3. 提示后续步骤
        print_header("部署完成")
        print_info("下一步:")
        print(f"  1. 启动服务器: python {sys.argv[0]} serve")
        print(f"  2. 新开终端测试: python {sys.argv[0]} test")
        print(f"\n或使用后台启动:")
        print(f"  nohup python {sys.argv[0]} serve > server.log 2>&1 &")


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print_info("\n用户中断")
        sys.exit(0)
    except Exception as e:
        print_error(f"程序错误: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)
```


从零实现 vLLM (1.3）：如何加速 Attention 计算：[https://mp.weixin.qq.com/s/BdWG6_ZTaGRknmsbGfFkMQ…](https://t.co/J49S4ccB3g) 从零实现 vLLM (1.2）：如何实现张量并行：[https://mp.weixin.qq.com/s/8BVEVPPqDQhQ2l8L90dMNQ…](https://t.co/wnbHusOQwO)
**从零实现 vLLM (1.1)：并行词嵌入**https://mp.weixin.qq.com/s/h1cFYDNxcHC30APcarF47A

llm = LLM(
    model=MODEL_PATH,
    tokenizer=MODEL_PATH,
    trust_remote_code=True,
    dtype="half",
    quantization="gptq",          # [模型量化](模型量化.md)的方式
    max_model_len=1024,
    tensor_parallel_size=max(1, torch.cuda.device_count()),
    gpu_memory_utilization=0.9,
    tensor_parallel_size=1,        # [GPU 显卡](GPU%20显卡.md)并行数量
)

    llm_config = {
        "model": MODEL_PATH,
        "trust_remote_code": True,
        "dtype": "half",                 # FP16 权重
        "max_model_len": 512,            # 从 2048 降到 512
        "gpu_memory_utilization": 0.72,  # 从 0.90 降到 0.72
        "tensor_parallel_size": tensor_parallel_size,
        "disable_log_stats": True,
        "swap_space": 8,                 # 8 GiB CPU 交换空间
        "kv_cache_dtype": "fp8",         # FP8 KV Cache（大幅省显存）
        "max_num_batched_tokens": 512,   # 控制批量总 token
        "max_num_seqs": 16,              # 控制并发序列
        "enforce_eager": True,           # 避免 CUDA graph 额外占用
        "distributed_executor_backend": "mp",   # ★ 强制用 MP，绕开 Ray
    }


vLLM 是一个开源的 Python 库，用于快速、高效地部署大型语言模型（LLMs）。它的主要目标是显著提升 LLMs 的推理速度和吞吐量，使其更适用于生产环境。

以下是 vLLM 的一些关键特点和功能：
- **高效的内存管理：**
    - vLLM 使用一种称为 [[PagedAttention]] 的创新技术，它可以有效地管理注意力机制中的内存，避免浪费。
    - Paged Attention 允许 vLLM 连续处理更长的序列，并支持更高的吞吐量。
- **快速推理：**
    - 通过优化的内核和高效的内存管理，vLLM 实现了比传统 LLM 服务框架更高的推理速度。
    - 通过最优的batching和cache机制实现了更高级的推理效率。
- **易于使用：**
    - vLLM 提供了一个简单的 API，可以轻松地将 LLMs 部署为服务。
    - 它支持各种流行的 LLM 架构，并且可以与 Hugging Face Transformers 等库集成。
- **支持多种解码算法：**
    - 支持各种解码算法，其中包括贪婪解码，采样，beam search等等。
- **应用场景：**
    - vLLM 适用于需要高性能 LLM 推理的各种应用程序，例如聊天机器人、文本生成和问答系统。

简而言之，vLLM 的主要优势在于它能够显著提高大型语言模型推理的效率，从而降低延迟并提高吞吐量，这使得它在大规模部署 LLM 时非常有价值。
