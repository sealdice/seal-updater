# seal-updater

海豹的升级程序，负责覆盖文件和重启

## 命令行参数

- `--upgrade` 要解压的安装包路径
- `--pid` 海豹主进程的 PID
- `--cwd` 工作路径，即解压到该路径下。默认为当前路径
- `--verbose` 打印更多信息
- `--skip-startup` 解压后，不启动海豹
