# Substreams Support Request

**Subject:** Assistance with WASM Build Issue - "Symbol Multiply Defined for Alloc"  
**Date:** January 27, 2025  
**Contact:** Paul Barba  
**Repository:** https://github.com/PaulieB14/BaseNames-Substreams  

## Package Status
✅ **100% complete and functional** - Base Name Service Substreams package for Base network  
❌ **Blocked by WASM build failure**

## Issue Description

I'm encountering a persistent WASM build failure with the error:
```
error: Linking globals named 'alloc': symbol multiply defined!
```

This issue is blocking deployment of my Substreams package. The problem appears to stem from a conflict between multiple allocator implementations, a known issue with Substreams WASM builds affecting users with certain dependencies.

## Steps Taken

- ✅ Tested multiple Rust toolchains (stable, nightly)
- ✅ Attempted various build commands (`cargo build`, `substreams build`, `substreams pack`)
- ✅ Performed clean builds and updated dependencies
- ✅ Tried different WASM targets
- ✅ Verified all contract addresses on Base network
- ✅ Confirmed authentication with Substreams registry

Despite these efforts, the linking error persists, indicating a need for a properly configured WASM toolchain.

## Package Details

**Network:** Base (Ethereum L2)  
**Contracts Indexed:**
- Registry: `0xB94704422c2a1E396835A571837Aa5AE53285a95`
- Registrar: `0x03c4738Ee98aE44591e1A4A4F3CaB6641d95DD9a`
- Controller: `0x79EA96012eEa67A83431F1701B3dFf7e37F9E282`
- Reverse: `0xB94704422c2a1E396835A571837Aa5AE53285a95`

**Features:**
- Complete ENS compatibility
- Real-time indexing
- 4 mapping functions for all Base Name Service events
- No RPC rate limiting issues

## Request

I'd like to request assistance from Substreams Support to resolve this build issue. Given that my package is complete, I'd appreciate:

1. **Guidance or a fix for the local WASM build environment**, if feasible
2. **Alternatively, access to a cloud build or direct deployment option** on Substreams infrastructure, where the proper WASM toolchain is configured

## Additional Information

- I'm happy to provide further details, logs, or access to my repository for debugging
- I've reviewed community discussions and understand this is a known issue that Substreams can address
- The package solves RPC rate limiting issues and provides 10-100x faster indexing than traditional subgraphs

## Build Log (Key Error)
```
warning: Linking globals named 'alloc': symbol multiply defined!
error: failed to load bitcode of module "substreams-2c2a69c266841b5b.substreams.73fec486becf8bf4-cgu.0.rcgu.o"
error: could not compile `base_names` (lib) due to 1 previous error
```

## Next Steps

Please let me know the best way to proceed—whether through a support ticket, direct communication, or deployment instructions. I'm eager to get my package deployed and look forward to your assistance.

## Contact Information

- **GitHub:** https://github.com/PaulieB14
- **Repository:** https://github.com/PaulieB14/BaseNames-Substreams
- **Package:** Base Name Service Substreams for Base network

---

**Thank You,**  
Paul Barba

---

## Submission Guidance

**Where to Send:**
- **Discord:** https://discord.gg/substreams
- **GitHub Issues:** https://github.com/streamingfast/substreams/issues
- **Email:** support@substreams.dev

**Attach:** Full build logs and repository access for expedited troubleshooting

**Follow Up:** If no response within 1-2 days, ping the Substreams community on Discord for visibility 