# Rust gRPC Tutorial

…

## Reflection

### 1. Key differences between unary, server-streaming, and bi-directional-streaming RPCs  
- **Unary** calls send one request and get one response. They’re simple, low-overhead, and ideal for single‐shot operations (e.g. fetching a user profile, authentication, one-off calculations).  
- **Server-streaming** calls send one request and receive a sequence of responses. They’re suited to long-running data feeds or large payloads split into manageable chunks (e.g. log tails, file downloads, transaction history).  
- **Bi-directional-streaming** lets client and server exchange independent message streams over the same connection. This enables real-time, two-way communication (e.g. chat, collaborative editing).  

### 2. Security considerations for Rust gRPC services  
- **Authentication & authorization**: integrate token-based schemes (JWT, OAuth2) at the interceptor layer to verify caller identity and enforce per-RPC access control.  
- **Encryption**: enable TLS/SSL for HTTP/2 channels (via `ServerTlsConfig` / `ChannelTlsConfig`) to protect data in transit.  
- **Denial-of-service**: enforce per-stream and per-message size limits, rate-limit incoming connections, and validate payloads to avoid resource exhaustion.  

### 3. Challenges in handling bidirectional streaming  
- **Backpressure & buffering**: coordinating client and server send/receive rates to avoid unbounded memory growth or dropped messages.  
- **Error propagation**: deciding whether a transient error on one side should tear down the entire stream or be handled locally.  
- **Lifecycle management**: gracefully shutting down streams when a peer disconnects or when the application shuts down.  

### 4. Advantages & disadvantages of `ReceiverStream`  
- **Advantages**  
  - Seamless adapter from a Tokio `mpsc::Receiver` into a `Stream` for tonic.  
  - Minimal boilerplate: just wrap your channel receiver.  
- **Disadvantages**  
  - Hard-limits on buffer size must be tuned manually.  
  - No built-in backpressure feedback—once the channel is full, `send().await` back-pressures the sender, but the overall flow control is at application level.  

### 5. Structuring Rust gRPC code for reuse and modularity  
- **Split services into separate crates or modules** (`services`, `client`, `server`, `common`).  
- **Define shared data types** and helper functions in a `common` crate.  
- **Use traits and generics** for cross‐cutting concerns (e.g. logging, error translation).  
- **Employ interceptors and middleware** for authentication and metrics, so service‐specific logic remains focused.  

### 6. Extending `MyPaymentService` for complex logic  
- Validate request parameters (e.g. non-zero amount, valid `user_id`).  
- Integrate with a database or external payment gateway (e.g. wrap in a transaction, handle retries, idempotency keys).  
- Implement audit logging and error handling for failed transactions.  
- Emit domain events (e.g. via Kafka) after successful payment.  

### 7. Impact of adopting gRPC on distributed systems architecture  
- **Strongly-typed contracts** (via .proto) enforce schema compatibility and auto-generate client/server stubs across languages.  
- **HTTP/2 multiplexing** reduces connection overhead and enables efficient stream interleaving.  
- **Cross-platform interoperability**: many ecosystems support gRPC (Go, Java, Python, C#), facilitating polyglot architectures.  
- **Steeper learning curve** compared to REST/JSON, and more boilerplate around .proto management.  

### 8. HTTP/2 vs HTTP/1.1 (and WebSocket) for APIs  
- **HTTP/2**  
  - Pros: multiplexed streams, header compression (HPACK), built-in flow control.  
  - Cons: requires TLS in most clients, more complex server setup.  
- **HTTP/1.1 + WebSocket**  
  - Pros: wider compatibility, simpler servers for basic setups.  
  - Cons: separate upgrade handshake, no native RPC framing, extra complexity in message serialization.  

### 9. REST request-response vs gRPC bidirectional streaming  
- **REST/JSON**: each request/response is independent, simple to cache and inspect. However, real-time updates require polling or WebSockets.  
- **gRPC streaming**: enables real-time push/pull patterns without polling. Clients can subscribe once and react to events immediately.  

### 10. Protocol Buffers vs JSON payloads  
- **Protobuf (schema-based)**  
  - Pros: compact binary wire format, forward/backward‐compatible schema evolution, codegen for multiple languages.  
  - Cons: harder to debug by hand, requires proto compiler step.  
- **JSON (schema-less)**  
  - Pros: human-readable, no build tooling for schemas.  
  - Cons: larger wire size, less strict type enforcement, schema drift risk.  
