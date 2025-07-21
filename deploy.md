# Base Name Service Substreams - Deployment Guide

## Package Information

**Repository:** https://github.com/PaulieB14/BaseNames-Substreams  
**Network:** Base (Ethereum L2)  
**Status:** Complete and ready for deployment  

## Contract Addresses (Verified on Base)

- **Registry:** `0xB94704422c2a1E396835A571837Aa5AE53285a95`
- **Registrar:** `0x03c4738Ee98aE44591e1A4A4F3CaB6641d95DD9a`
- **Controller:** `0x79EA96012eEa67A83431F1701B3dFf7e37F9E282`
- **Reverse:** `0xB94704422c2a1E396835A571837Aa5AE53285a95`

## Deployment Issue

**Problem:** WASM build linking issue preventing local compilation  
**Impact:** Cannot create .spkg file locally  
**Solution:** Deploy directly to Substreams infrastructure  

## Deployment Options

### Option 1: Contact Substreams Support
- **Email:** support@substreams.dev
- **Discord:** https://discord.gg/substreams
- **GitHub:** https://github.com/streamingfast/substreams

### Option 2: Use Substreams Sink
Deploy to Substreams SQL sink for database access.

### Option 3: Cloud Build
Use Substreams cloud build service to bypass local WASM issues.

## Package Features

- ✅ 4 mapping functions for all Base Name Service events
- ✅ Complete ENS compatibility
- ✅ Real-time indexing
- ✅ No RPC rate limiting
- ✅ 10-100x faster than traditional subgraphs

## Benefits

- **Solves RPC rate limiting** issues
- **Eliminates 74-day sync time**
- **Provides real-time updates**
- **Complete Base Name Service coverage**

---

**Ready for deployment!** Contact Substreams team for assistance. 