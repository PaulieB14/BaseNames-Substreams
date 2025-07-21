# Base Name Service Substreams

A Substreams package for indexing Base Name Service (Base's ENS-compatible naming system) on the Base network.

## 🚀 Features

- **Full ENS Compatibility**: Indexes all Base Name Service events (registrations, transfers, reverse resolution)
- **Real-time Processing**: Processes events as they happen on Base network
- **No RPC Rate Limiting**: Uses Substreams for efficient data processing
- **GraphQL Output**: Generates GraphQL-compatible entity changes
- **Reverse Resolution**: Supports wallet address to domain name lookups

## 📋 Prerequisites

- [Substreams CLI](https://substreams.dev/docs/develop/getting-started)
- [Rust](https://rustup.rs/) (for building)
- [Graph CLI](https://thegraph.com/docs/en/developing/installation/) (for deployment)

## 🏗️ Building

```bash
# Build the Substreams package
cargo build --target wasm32-unknown-unknown --release

# Generate protobuf code
cargo build
```

## 🚀 Deployment

### 1. Deploy to Substreams Sink

```bash
# Deploy to The Graph's Substreams Sink
substreams deploy \
  --package-path substreams.yaml \
  --endpoint https://base.substreams.pinax.network:443 \
  --network base \
  --start-block 0
```

### 2. Deploy to Graph Studio

```bash
# Deploy to Graph Studio for GraphQL API
graph deploy \
  --product hosted-service \
  --node https://api.studio.thegraph.com/deploy/ \
  --ipfs https://api.thegraph.com/ipfs/ \
  base-names-substreams
```

## 📊 Data Sources

The Substreams package indexes events from these Base Name Service contracts:

- **Registry**: `0x03c4738ee98ae44591e1a4a4f3cab6641d95dd9a`
- **BaseRegistrar**: `0x4ccb0bb02fcaba27e82a56646e81d8c5bc4119a5`
- **RegistrarController**: `0x79ea96012eea67a83431f1701b3dff7e37f9e282`
- **ReverseRegistrar**: `0xb94704422c2a1e396835a571837aa5ae53285a95`

## 🔍 Indexed Events

### Registry Events
- `NewOwner`: Domain ownership transfers
- `NewResolver`: Resolver contract assignments
- `NewTTL`: TTL updates

### Registrar Events
- `NameRegistered`: New domain registrations
- `NameRenewed`: Domain renewals
- `Transfer`: NFT transfers

### Controller Events
- `NameRegistered`: Registration events from controller
- `NameRenewed`: Renewal events from controller

### Reverse Events
- `ReverseClaimed`: Reverse resolution claims

## 📈 Performance Benefits

Compared to traditional subgraphs:

- **10-100x faster** indexing
- **No RPC rate limiting** issues
- **Real-time processing** from deployment
- **Efficient data streaming**

## 🔧 Configuration

### Environment Variables

```bash
# Base network RPC (optional - Substreams handles this)
BASE_RPC_URL=https://mainnet.base.org

# Substreams endpoint
SUBSTREAMS_ENDPOINT=https://base.substreams.pinax.network:443
```

### Start Block

The package starts indexing from block 0 to capture all historical Base Name Service data.

## 📝 Usage Examples

### Query Domains

```graphql
{
  domains(first: 10) {
    id
    name
    labelName
    owner
    resolver
  }
}
```

### Query Registrations

```graphql
{
  registrations(first: 10) {
    id
    domain {
      id
      name
    }
    registrationDate
    expiryDate
    registrant
  }
}
```

### Reverse Resolution

```graphql
{
  reverseRegistrations(first: 10) {
    id
    domain {
      id
      name
    }
    address
  }
}
```

## 🛠️ Development

### Local Testing

```bash
# Run locally with Substreams GUI
substreams gui substreams.yaml map_registry_events

# Test with sample data
substreams run substreams.yaml map_registry_events \
  --start-block 18000000 \
  --end-block 18000100
```

### Adding New Events

1. Add event signature to constants in `src/lib.rs`
2. Create parsing function
3. Add to appropriate mapping function
4. Update protobuf definitions if needed

## 📚 Resources

- [Base Name Service Documentation](https://docs.basename.xyz/)
- [Substreams Documentation](https://substreams.dev/docs/)
- [The Graph Documentation](https://thegraph.com/docs/)

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## 📄 License

MIT License - see LICENSE file for details. 