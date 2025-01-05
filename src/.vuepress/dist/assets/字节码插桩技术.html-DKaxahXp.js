import{_ as n}from"./plugin-vue_export-helper-DlAUqK2U.js";import{c as a,f as i,o as e}from"./app-DokaGNO4.js";const l={};function p(t,s){return e(),a("div",null,s[0]||(s[0]=[i(`<h1 id="典型回答" tabindex="-1"><a class="header-anchor" href="#典型回答"><span>典型回答</span></a></h1><p>Arthas在统计方法耗时时，使用了<strong>字节码插桩技术</strong>。它会在需要监控的方法中插入代码，在方法执行前记录开始时间，在方法执行完毕后记录结束时间，并计算两者的差值得到方法执行时间。这个过程就是字节码插桩的经典应用之一。</p><p>相比传统的基于AspectJ等框架实现AOP的方式，Arthas的动态插桩能力更强，支持无侵入式的监控。</p><h1 id="扩展知识" tabindex="-1"><a class="header-anchor" href="#扩展知识"><span>扩展知识</span></a></h1><h2 id="字节码插桩" tabindex="-1"><a class="header-anchor" href="#字节码插桩"><span>字节码插桩</span></a></h2><p>Java字节码插桩技术是指在编译期或运行期，通过修改Java字节码的方式，向代码中插入额外的代码，<strong>它可以在不改变Java源代码的情况下，对Java应用程序的运行时行为进行监控、调试、分析和优化等</strong>。例如实现性能监控、代码覆盖率检测、代码安全扫描等。</p><p>字节码插桩技术通常包括以下几个步骤：</p><ol><li>生成目标类的字节码，这可以通过Java编译器（如javac）或其他工具（如AspectJ）完成。</li><li>解析字节码，识别需要插桩的代码区域（如方法、循环、异常处理等）。</li><li>插入额外的字节码，这些字节码通常是通过编写Java代码来实现的，并通过字节码生成库（如ASM、Javassist等）生成对应的字节码。</li><li>将修改后的字节码重新写回到磁盘或内存中，以便后续使用。</li></ol><p>假设我们需要对一个Java方法进行性能监控，我们可以在方法的入口和出口处分别插入计时器，来统计方法的执行时间。这可以通过以下代码实现：</p><div class="language-plain line-numbers-mode" data-highlighter="shiki" data-ext="plain" data-title="plain" style="--shiki-light:#383A42;--shiki-dark:#abb2bf;--shiki-light-bg:#FAFAFA;--shiki-dark-bg:#282c34;"><pre class="shiki shiki-themes one-light one-dark-pro vp-code"><code><span class="line"><span>public class Monitor {</span></span>
<span class="line"><span>    public static void start() {</span></span>
<span class="line"><span>        long startTime = System.nanoTime();</span></span>
<span class="line"><span>        // 将起始时间记录到ThreadLocal中，以便在方法返回时进行计算</span></span>
<span class="line"><span>        ThreadLocalHolder.set(&quot;startTime&quot;, startTime);</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    public static void end() {</span></span>
<span class="line"><span>        long endTime = System.nanoTime();</span></span>
<span class="line"><span>        // 获取起始时间</span></span>
<span class="line"><span>        long startTime = (long) ThreadLocalHolder.get(&quot;startTime&quot;);</span></span>
<span class="line"><span>        // 计算方法执行时间</span></span>
<span class="line"><span>        long elapsedTime = endTime - startTime;</span></span>
<span class="line"><span>        System.out.println(&quot;Method execution time: &quot; + elapsedTime + &quot;ns&quot;);</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>}</span></span>
<span class="line"><span></span></span>
<span class="line"><span>public class Example {</span></span>
<span class="line"><span>    public void method() {</span></span>
<span class="line"><span>        Monitor.start();</span></span>
<span class="line"><span>        // 执行方法逻辑</span></span>
<span class="line"><span>        Monitor.end();</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>}</span></span></code></pre><div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0;"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div><p>但是，如果需要对多个方法进行性能监控，就需要在每个方法中分别插入Monitor.start()和Monitor.end()，这样会导致代码重复，可读性差，并且容易漏掉一些方法。这时，我们就可以使用字节码插桩技术，在编译期或者运行期，自动向每个方法的入口和出口处插入Monitor.start()和Monitor.end()，来实现代码的统一性和可维护性。</p><p>具体实现可以使用字节码生成库ASM或Javassist来实现，这里以ASM为例。下面的代码演示了如何使用ASM对Example类进行字节码插桩：</p><div class="language-plain line-numbers-mode" data-highlighter="shiki" data-ext="plain" data-title="plain" style="--shiki-light:#383A42;--shiki-dark:#abb2bf;--shiki-light-bg:#FAFAFA;--shiki-dark-bg:#282c34;"><pre class="shiki shiki-themes one-light one-dark-pro vp-code"><code><span class="line"><span>import org.objectweb.asm.ClassReader;</span></span>
<span class="line"><span>import org.objectweb.asm.ClassVisitor;</span></span>
<span class="line"><span>import org.objectweb.asm.ClassWriter;</span></span>
<span class="line"><span>import org.objectweb.asm.MethodVisitor;</span></span>
<span class="line"><span>import org.objectweb.asm.Opcodes;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>import java.io.IOException;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>public class MonitorTransformer implements Opcodes {</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    public static byte[] transform(byte[] classBytes) throws IOException {</span></span>
<span class="line"><span>        ClassReader reader = new ClassReader(classBytes);</span></span>
<span class="line"><span>        ClassWriter writer = new ClassWriter(ClassWriter.COMPUTE_MAXS | ClassWriter.COMPUTE_FRAMES);</span></span>
<span class="line"><span>        ClassVisitor visitor = new ClassVisitor(Opcodes.ASM5, writer) {</span></span>
<span class="line"><span>            @Override</span></span>
<span class="line"><span>            public MethodVisitor visitMethod(int access, String name, String desc, String signature, String[] exceptions) {</span></span>
<span class="line"><span>                MethodVisitor mv = super.visitMethod(access, name, desc, signature, exceptions);</span></span>
<span class="line"><span>                // 只为指定方法添加字节码插桩</span></span>
<span class="line"><span>                if (&quot;method&quot;.equals(name) &amp;&amp; &quot;()V&quot;.equals(desc)) {</span></span>
<span class="line"><span>                    mv = new MethodVisitor(Opcodes.ASM5, mv) {</span></span>
<span class="line"><span>                        @Override</span></span>
<span class="line"><span>                        public void visitCode() {</span></span>
<span class="line"><span>                            super.visitCode();</span></span>
<span class="line"><span>                            // 在方法执行之前插入字节码</span></span>
<span class="line"><span>                            mv.visitMethodInsn(INVOKESTATIC, &quot;Monitor&quot;, &quot;start&quot;, &quot;()V&quot;, false);</span></span>
<span class="line"><span>                        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>                        @Override</span></span>
<span class="line"><span>                        public void visitInsn(int opcode) {</span></span>
<span class="line"><span>                            // 在方法返回之前插入字节码</span></span>
<span class="line"><span>                            if (opcode == RETURN) {</span></span>
<span class="line"><span>                                mv.visitMethodInsn(INVOKESTATIC, &quot;Monitor&quot;, &quot;end&quot;, &quot;()V&quot;, false);</span></span>
<span class="line"><span>                            }</span></span>
<span class="line"><span>                            super.visitInsn(opcode);</span></span>
<span class="line"><span>                        }</span></span>
<span class="line"><span>                    };</span></span>
<span class="line"><span>                }</span></span>
<span class="line"><span>                return mv;</span></span>
<span class="line"><span>            }</span></span>
<span class="line"><span>        };</span></span>
<span class="line"><span>        reader.accept(visitor, ClassReader.EXPAND_FRAMES);</span></span>
<span class="line"><span>        return writer.toByteArray();</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>}</span></span></code></pre><div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0;"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div>`,13)]))}const c=n(l,[["render",p],["__file","字节码插桩技术.html.vue"]]),o=JSON.parse('{"path":"/2%20%E7%AC%AC%E4%BA%8C%E5%A4%A7%E8%84%91/1%20%E8%8A%82%E7%82%B9/CS/%E7%BC%96%E7%A8%8B%E8%AF%AD%E8%A8%80/Java/%E5%AD%97%E8%8A%82%E7%A0%81%E6%8F%92%E6%A1%A9%E6%8A%80%E6%9C%AF.html","title":"典型回答","lang":"zh-CN","frontmatter":{"description":"典型回答 Arthas在统计方法耗时时，使用了字节码插桩技术。它会在需要监控的方法中插入代码，在方法执行前记录开始时间，在方法执行完毕后记录结束时间，并计算两者的差值得到方法执行时间。这个过程就是字节码插桩的经典应用之一。 相比传统的基于AspectJ等框架实现AOP的方式，Arthas的动态插桩能力更强，支持无侵入式的监控。 扩展知识 字节码插桩 J...","head":[["meta",{"property":"og:url","content":"https://vuepress-theme-hope-docs-demo.netlify.app/2%20%E7%AC%AC%E4%BA%8C%E5%A4%A7%E8%84%91/1%20%E8%8A%82%E7%82%B9/CS/%E7%BC%96%E7%A8%8B%E8%AF%AD%E8%A8%80/Java/%E5%AD%97%E8%8A%82%E7%A0%81%E6%8F%92%E6%A1%A9%E6%8A%80%E6%9C%AF.html"}],["meta",{"property":"og:site_name","content":"转了码的刘公子"}],["meta",{"property":"og:title","content":"典型回答"}],["meta",{"property":"og:description","content":"典型回答 Arthas在统计方法耗时时，使用了字节码插桩技术。它会在需要监控的方法中插入代码，在方法执行前记录开始时间，在方法执行完毕后记录结束时间，并计算两者的差值得到方法执行时间。这个过程就是字节码插桩的经典应用之一。 相比传统的基于AspectJ等框架实现AOP的方式，Arthas的动态插桩能力更强，支持无侵入式的监控。 扩展知识 字节码插桩 J..."}],["meta",{"property":"og:type","content":"article"}],["meta",{"property":"og:locale","content":"zh-CN"}],["meta",{"property":"og:updated_time","content":"2024-12-11T14:48:27.000Z"}],["meta",{"property":"article:modified_time","content":"2024-12-11T14:48:27.000Z"}],["script",{"type":"application/ld+json"},"{\\"@context\\":\\"https://schema.org\\",\\"@type\\":\\"Article\\",\\"headline\\":\\"典型回答\\",\\"image\\":[\\"\\"],\\"dateModified\\":\\"2024-12-11T14:48:27.000Z\\",\\"author\\":[{\\"@type\\":\\"Person\\",\\"name\\":\\"转了码的刘公子\\",\\"url\\":\\"https://mister-hope.com\\"}]}"]]},"headers":[{"level":2,"title":"字节码插桩","slug":"字节码插桩","link":"#字节码插桩","children":[]}],"git":{"createdTime":1733928507000,"updatedTime":1733928507000,"contributors":[{"name":"Luis","email":"liuysh20@gmail.com","commits":1}]},"readingTime":{"minutes":2.86,"words":858},"filePathRelative":"2 第二大脑/1 节点/CS/编程语言/Java/字节码插桩技术.md","localizedDate":"2024年12月11日","autoDesc":true}');export{c as comp,o as data};
