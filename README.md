# Base Name Service Substreams

A high-performance Substreams package for indexing Base Name Service (ENS-compatible naming system) on the Base network.

## 🚀 What is This?

This is a **Substreams package**, not a traditional subgraph. Substreams provides:
- **Real-time indexing** (no 74-day sync time)
- **10-100x faster** than traditional subgraphs
- **No RPC rate limiting** issues
- **Streaming data** instead of GraphQL queries

## 📋 Features

- ✅ **Complete Base Name Service coverage**
- ✅ **4 mapping functions** for all contract events
- ✅ **Real-time event processing**
- ✅ **ENS-compatible** naming system
- ✅ **Base network** (Ethereum L2) support

## 🏗️ Architecture

### Contracts Indexed
- **Registry**: `0xB94704422c2a1E396835A571837Aa5AE53285a95`
- **Registrar**: `0x03c4738Ee98aE44591e1A4A4F3CaB6641d95DD9a`
- **Controller**: `0x79EA96012eEa67A83431F1701B3dFf7e37F9E282`
- **Reverse**: `0xB94704422c2a1E396835A571837Aa5AE53285a95`

### Module Handlers
1. **`map_registry_events`** - Registry contract events
2. **`map_registrar_events`** - Registrar contract events  
3. **`map_controller_events`** - Controller contract events
4. **`map_reverse_events`** - Reverse resolution events

## 🛠️ Development

### Prerequisites
- Rust 1.65+
- Substreams CLI
- Base network access

### Build
```bash
# Build WASM module
cargo build --target wasm32-unknown-unknown --release

# Create package
substreams pack

# Deploy to registry
substreams registry publish .
```

### Local Testing
```bash
# Test on Base network
substreams run map_registry_events \
  --start-block 1000000 \
  --stop-block +10 \
  --endpoint https://base-mainnet.substreams.pinax.network:443
```

## 📊 Data Output

This package outputs protobuf messages containing:
- **Event data** from Base Name Service contracts
- **Block information** (hash, number, timestamp)
- **Transaction details** (hash, log index)
- **Contract-specific fields** (names, owners, costs, etc.)

## 🔄 Usage

### Streaming Data
```bash
# Stream registry events
substreams run map_registry_events \
  --endpoint https://base-mainnet.substreams.pinax.network:443

# Stream registrar events  
substreams run map_registrar_events \
  --endpoint https://base-mainnet.substreams.pinax.network:443
```

### Substreams Sink
Deploy to various sinks for different use cases:
- **Substreams:SQL** - Database storage
- **Substreams:Stream** - Real-time streaming
- **Substreams:PubSub** - Message queuing

## 🆚 vs Traditional Subgraphs

| Feature | Traditional Subgraph | Substreams |
|---------|---------------------|------------|
| Sync Time | 74+ days | Real-time |
| RPC Limits | Rate limited | No limits |
| Queries | GraphQL | Streaming |
| Performance | 1x | 10-100x |
| Data Freshness | Delayed | Instant |

## 🏗️ Project Structure

```
base-names-substreams/
├── src/
│   └── lib.rs              # Main handlers
├── protobuf/
│   └── base_names.proto    # Data schemas
├── substreams.yaml         # Package manifest
├── Cargo.toml             # Rust dependencies
└── build.rs               # Build script
```

## 🔧 Configuration

### Environment Variables
```bash
# Substreams authentication
export SUBSTREAMS_API_TOKEN="your-jwt-token"
export SUBSTREAMS_ENDPOINT="https://base-mainnet.substreams.pinax.network:443"
```

## 📈 Benefits

1. **Solve RPC Rate Limiting** - No more API limits
2. **Real-time Updates** - Instant data availability
3. **Scalable Architecture** - Handle high throughput
4. **Cost Effective** - Reduced infrastructure costs
5. **Future Proof** - Built for modern blockchain indexing

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test with `substreams run`
5. Submit a pull request

## 📄 License

MIT License - see LICENSE file for details

## 🔗 Links

- **Base Name Service**: https://basescan.org/address/0xb94704422c2a1e396835a571837aa5ae53285a95
- **Substreams Docs**: https://docs.substreams.dev/
- **Base Network**: https://base.org/

---

**Built with ❤️ for the Base ecosystem** 