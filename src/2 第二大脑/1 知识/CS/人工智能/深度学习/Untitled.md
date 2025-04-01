---
draw:
title: Untitled
date created: 2025-03-24
date modified: 2025-03-24
---

penr1_py311) root@autodl-container-c2b244be67-2c84c722:~/autodl-tmp/open-r1# accelerate launch --config_file=recipes/accelerate_configs/zero3.yaml src/open_r1/sft.py     --model_name_or_path Qwen/Qwen2.5-1.5B-Instruct     --dataset_name /root/autodl-tmp/open-r1/data/OpenR1-Math-220k     --learning_rate 1.0e-5     --num_train_epochs 1     --packing     --max_seq_length 2048     --per_device_train_batch_size 32     --fp16     --output_dir data/Qwen2.5-1.5B-Open-R1-Distill     --resume_from_checkpoint data/Qwen2.5-1.5B-Open-R1-Distill/checkpoint-14500     --save_steps 200000

[2025-03-24 11:50:10,050][INFO][real_accelerator.py:219:get_accelerator]Setting ds_accelerator to cuda (auto detect)

[2025-03-24 11:50:14,792][INFO][real_accelerator.py:219:get_accelerator]Setting ds_accelerator to cuda (auto detect)

[2025-03-24 11:50:15,441][INFO][comm.py:652:init_distributed]cdb=None

[2025-03-24 11:50:15,442][INFO][comm.py:683:init_distributed]Initializing TorchBackend in DeepSpeed with backend nccl

Resolving data files: 100%|██████████████████████████████████████████████████████████████████| 20/20[00:00<00:00, 235635.06it/s]

[2025-03-24 11:50:36,150][INFO][config.py:733:__init__]Config mesh_device None world_size = 1

[WARNING|logging.py:329]2025-03-24 11:50:36,491 >> Sliding Window Attention is enabled but not implemented for `sdpa`; unexpected results may be encountered.

[2025-03-24 11:50:36,616][INFO][partition_parameters.py:348:__exit__]finished initializing model - num_params = 339, num_elems = 1.78B

Parameter Offload: Total persistent parameters: 144896 in 141 params

/root/miniconda3/envs/openr1_py311/lib/python3.11/site-packages/deepspeed/runtime/checkpoint_engine/torch_checkpoint_engine.py:28: FutureWarning: You are using `torch.load` with `weights_only=False` (the current default value), which uses the default pickle module implicitly. It is possible to construct malicious pickle data which will execute arbitrary code during unpickling (See https://github.com/pytorch/pytorch/blob/main/SECURITY.md#untrusted-models for more details). In a future release, the default value for `weights_only` will be flipped to `True`. This limits the functions that could be executed during unpickling. Arbitrary objects will no longer be allowed to be loaded via this mode unless they are explicitly allowlisted by the user via `torch.serialization.add_safe_globals`. We recommend you start setting `weights_only=True` for any use case where you don't have full control of the loaded file. Please open an issue on GitHub for any issues related to this experimental feature.

  partition = torch.load(path, map_location=map_location)

[WARNING|logging.py:329]2025-03-24 11:50:58,379 >> Warning: The following arguments do not match the ones in the `trainer_state.json` within the checkpoint directory:

        save_steps: 200000 (from args) != 500 (from trainer_state.json)

        per_device_train_batch_size: 32 (from args) != 2 (from trainer_state.json)

wandb: WARNING The `run_name` is currently set to the same value as `TrainingArguments.output_dir`. If this was not intended, please specify a different run name by setting the `TrainingArguments.run_name` parameter.

wandb: Using wandb-core as the SDK backend.  Please refer to https://wandb.me/wandb-core for more information.

wandb: Currently logged in as: liuysh20 (liuysh20-qunar-com) to https://api.wandb.ai. Use `wandb login --relogin` to force relogin

wandb: Tracking run with wandb version 0.19.8

wandb: Run data is saved locally in /root/autodl-tmp/open-r1/wandb/run-20250324_115059-x08y3pcd

wandb: Run `wandb offline` to turn off syncing.

wandb: Syncing run data/Qwen2.5-1.5B-Open-R1-Distill

wandb: ⭐️ View project at https://wandb.ai/liuysh20-qunar-com/huggingface

wandb: 🚀 View run at https://wandb.ai/liuysh20-qunar-com/huggingface/runs/x08y3pcd

  0%|                                                                                                 | 0/137036[00:00<?,?it/s]/root/miniconda3/envs/openr1_py311/lib/python3.11/site-packages/transformers/trainer.py:3119: FutureWarning: You are using `torch.load` with `weights_only=False` (the current default value), which uses the default pickle module implicitly. It is possible to construct malicious pickle data which will execute arbitrary code during unpickling (See https://github.com/pytorch/pytorch/blob/main/SECURITY.md#untrusted-models for more details). In a future release, the default value for `weights_only` will be flipped to `True`. This limits the functions that could be executed during unpickling. Arbitrary objects will no longer be allowed to be loaded via this mode unless they are explicitly allowlisted by the user via `torch.serialization.add_safe_globals`. We recommend you start setting `weights_only=True` for any use case where you don't have full control of the loaded file. Please open an issue on GitHub for any issues related to this experimental feature.

  checkpoint_rng_state = torch.load(rng_file)

{'loss': 0.6719, 'grad_norm': 1.9568877136239657, 'learning_rate': 8.907440380629908e-06, 'mean_token_accuracy': 0.7970283323526383, 'epoch': 0.11}

{'loss': 0.6714, 'grad_norm': 1.7798449670413534, 'learning_rate': 8.87095361802738e-06, 'mean_token_accuracy': 0.7969938894510269, 'epoch': 0.11}

{'loss': 0.6712, 'grad_norm': 2.355433222777971, 'learning_rate': 8.834539828950057e-06, 'mean_token_accuracy': 0.7971563818454742, 'epoch': 0.12}

{'loss': 0.6738, 'grad_norm': 1.9425191217973103, 'learning_rate': 8.79805306634753e-06, 'mean_token_accuracy': 0.7962823988199234, 'epoch': 0.12}

{'loss': 0.6724, 'grad_norm': 2.081513294754642, 'learning_rate': 8.761566303745001e-06, 'mean_token_accuracy': 0.7964669264554978, 'epoch': 0.12}

{'loss': 0.673, 'grad_norm': 2.0778279097395007, 'learning_rate': 8.725152514667679e-06, 'mean_token_accuracy': 0.7964542039632797, 'epoch': 0.13}

{'loss': 0.6602, 'grad_norm': 2.0168899994134213, 'learning_rate': 8.688665752065151e-06, 'mean_token_accuracy': 0.7996996049880981, 'epoch': 0.13}

{'loss': 0.6644, 'grad_norm': 1.6956533721331188, 'learning_rate': 8.652178989462625e-06, 'mean_token_accuracy': 0.7992387127876281, 'epoch': 0.14}

{'loss': 0.6776, 'grad_norm': 1.5935404364298413, 'learning_rate': 8.6157652003853e-06, 'mean_token_accuracy': 0.7945327297449112, 'epoch': 0.14}

{'loss': 0.6642, 'grad_norm': 2.0813443387931545, 'learning_rate': 8.579278437782773e-06, 'mean_token_accuracy': 0.7988644893169403, 'epoch': 0.14}

{'loss': 0.656, 'grad_norm': 1.70054974074063, 'learning_rate': 8.54286464870545e-06, 'mean_token_accuracy': 0.8003408962488174, 'epoch': 0.15}

{'loss': 0.6569, 'grad_norm': 1.8151638499458107, 'learning_rate': 8.506377886102922e-06, 'mean_token_accuracy': 0.8002785797119141, 'epoch': 0.15}

{'loss': 0.6586, 'grad_norm': 1.7702731030232672, 'learning_rate': 8.469891123500394e-06, 'mean_token_accuracy': 0.7998319256305695, 'epoch': 0.15}

{'loss': 0.652, 'grad_norm': 2.084443559145768, 'learning_rate': 8.433404360897866e-06, 'mean_token_accuracy': 0.801823907494545, 'epoch': 0.16}

{'loss': 0.6648, 'grad_norm': 2.1076784076747024, 'learning_rate': 8.396990571820545e-06, 'mean_token_accuracy': 0.7986520196199417, 'epoch': 0.16}

{'loss': 0.6526, 'grad_norm': 2.3317089401710884, 'learning_rate': 8.360576782743221e-06, 'mean_token_accuracy': 0.8016990796327591, 'epoch': 0.16}

{'loss': 0.6573, 'grad_norm': 1.88172612289279, 'learning_rate': 8.324090020140693e-06, 'mean_token_accuracy': 0.8001887369155883, 'epoch': 0.17}

{'loss': 0.6566, 'grad_norm': 1.895942415795127, 'learning_rate': 8.287603257538165e-06, 'mean_token_accuracy': 0.8000968611240387, 'epoch': 0.17}

{'loss': 0.6443, 'grad_norm': 1.8886965731424155, 'learning_rate': 7.777080475203597e-06, 'mean_token_accuracy': 0.8033621909618378, 'epoch': 0.22}

{'loss': 0.6464, 'grad_norm': 1.8488744562620074, 'learning_rate': 7.740593712601069e-06, 'mean_token_accuracy': 0.8028452920913697, 'epoch': 0.23}

{'loss': 0.6443, 'grad_norm': 1.8886965731424155, 'learning_rate': 7.777080475203597e-06, 'mean_token_accuracy': 0.8033621909618378, 'epoch': 0.22}

{'loss': 0.6464, 'grad_norm': 1.8488744562620074, 'learning_rate': 7.740593712601069e-06, 'mean_token_accuracy': 0.8028452920913697, 'epoch': 0.23}

{'loss': 0.6487, 'grad_norm': 1.9789400347377648, 'learning_rate': 7.704179923523745e-06, 'mean_token_accuracy': 0.8019500542879104, 'epoch': 0.23}

{'loss': 0.6441, 'grad_norm': 1.8167730403047282, 'learning_rate': 7.667693160921219e-06, 'mean_token_accuracy': 0.8028330911397934, 'epoch': 0.23}

{'loss': 0.6501, 'grad_norm': 1.5508971568981231, 'learning_rate': 7.631279371843896e-06, 'mean_token_accuracy': 0.8012431561946869, 'epoch': 0.24}

{'loss': 0.6402, 'grad_norm': 1.8462165115081144, 'learning_rate': 7.594792609241368e-06, 'mean_token_accuracy': 0.8046822032928467, 'epoch': 0.24}

{'loss': 0.6423, 'grad_norm': 1.8728223442783742, 'learning_rate': 7.558378820164045e-06, 'mean_token_accuracy': 0.8036312119960785, 'epoch': 0.24}

{'loss': 0.6439, 'grad_norm': 1.8623879183749215, 'learning_rate': 7.521892057561517e-06, 'mean_token_accuracy': 0.8036719595193863, 'epoch': 0.25}

{'loss': 0.6451, 'grad_norm': 1.7584918504524765, 'learning_rate': 7.48540529495899e-06, 'mean_token_accuracy': 0.8029183614253997, 'epoch': 0.25}

 25%|████████████████████▏                                                           | 34500/137036[3:11:33<15:20:52,  1.86it/s]
