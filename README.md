# actions

learn github actions

[docs](https://docs.github.com/cn/actions)
[github action market](https://github.com/marketplace?type=actions)
[awesome-actions](https://github.com/sdras/awesome-actions)

[action](https://jasonkayzk.github.io/2020/08/28/Github-Actions%E6%80%BB%E7%BB%93/)
[action](https://www.cnblogs.com/kongyijilafumi/p/16149434.html)
[action](https://www.cnblogs.com/wujianbufengsao/articles/13572397.html)
[action](https://zhuanlan.zhihu.com/p/363551389)
[action](https://juejin.cn/post/6997442468216061966)
[action 进阶](https://www.jianshu.com/p/022086076190)

```shell
# 生成公私钥, 私钥配置到 github->action, 公钥写到部署机器的 ~/.ssh/authorized_keys 文件中
ssh-keygen -f ~/.ssh/deploy_rsa -t rsa -b 4096
ssh-keygen -m PEM -t rsa -b 4096
```

可能遇到的问题 Load key "/home/runner/.ssh/deploy_key": error in libcrypto

Added public key to authorized_keys and added a new line to private key.
意思是需要把公钥加到 authorized_keys 文件中, 在配置 actions 的时候需要在私钥文件最后 -----END OPENSSH PRIVATE KEY----- 加个回车换行, 然后 action 就能正确加载私钥了。 参考 [issue](https://github.com/easingthemes/ssh-deploy/issues/143).

另一个可能遇到的问题是 rsync: connection unexpectedly closed (0 bytes received so far) [sender], 这时需要在要部署的主机上执行 yum install rsync 或者 sudo apt-get install -y rsync.
