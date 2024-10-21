# 使用cargo expand
先注意在toml中添加bin；
cargo expand --bin callee > expanded_code.rs> tests/expanded_code.rs

# 设计思路
其实不用让发送端和接收端去考虑多节点的问题， 只需要让发送端往一个地址发，接收端在一个地址监听就行。
接下来设计一个负载均衡器， 只需要让所有发送端都向这个均衡器发， 然后由均衡器进行派送即可。这里我由用uuid进行消息的管理，之后可以用uuid来返回即可