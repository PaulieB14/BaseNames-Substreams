#!/usr/bin/env python3
"""
Test script for Base Name Service contracts on Base network
"""

import requests
import json
from web3 import Web3

# Base RPC endpoint
BASE_RPC = "https://mainnet.base.org"

# Base Name Service contract addresses on Base
REGISTRY_ADDRESS = "0xb94704422c2a1e396835a571837aa5ae53285a95"
REGISTRAR_ADDRESS = "0x03c4738ee98ae44591e1a4a4f3cab6641d95dd9a"
CONTROLLER_ADDRESS = "0x79ea96012eea67a83431f1701b3dff7e37f9e282"
REVERSE_ADDRESS = "0xb94704422c2a1e396835a571837aa5ae53285a95"

def test_base_connection():
    """Test connection to Base network"""
    try:
        w3 = Web3(Web3.HTTPProvider(BASE_RPC))
        latest_block = w3.eth.block_number
        print(f"✅ Connected to Base network")
        print(f"📦 Latest block: {latest_block}")
        return True
    except Exception as e:
        print(f"❌ Failed to connect to Base network: {e}")
        return False

def test_contract_addresses():
    """Test if contract addresses are valid"""
    addresses = {
        "Registry": REGISTRY_ADDRESS,
        "Registrar": REGISTRAR_ADDRESS,
        "Controller": CONTROLLER_ADDRESS,
        "Reverse": REVERSE_ADDRESS
    }
    
    w3 = Web3(Web3.HTTPProvider(BASE_RPC))
    
    for name, address in addresses.items():
        try:
            # Convert to checksum address
            checksum_address = w3.to_checksum_address(address)
            # Check if address is valid
            if w3.is_address(checksum_address):
                # Try to get code at address
                code = w3.eth.get_code(checksum_address)
                if code != b'':
                    print(f"✅ {name}: {checksum_address} (has code)")
                else:
                    print(f"⚠️  {name}: {checksum_address} (no code)")
            else:
                print(f"❌ {name}: {checksum_address} (invalid address)")
        except Exception as e:
            print(f"❌ {name}: {address} (error: {e})")

def test_recent_events():
    """Test for recent Base Name Service events"""
    try:
        # Get recent blocks
        w3 = Web3(Web3.HTTPProvider(BASE_RPC))
        latest_block = w3.eth.block_number
        
        print(f"\n🔍 Checking recent blocks for Base Name Service events...")
        print(f"📦 Checking blocks {latest_block - 100} to {latest_block}")
        
        # Check last 100 blocks for events
        event_found = False
        for block_num in range(latest_block - 100, latest_block + 1):
            try:
                block = w3.eth.get_block(block_num, full_transactions=True)
                for tx in block.transactions:
                    if tx.to and tx.to.lower() in [
                        w3.to_checksum_address(REGISTRY_ADDRESS).lower(),
                        w3.to_checksum_address(REGISTRAR_ADDRESS).lower(),
                        w3.to_checksum_address(CONTROLLER_ADDRESS).lower(),
                        w3.to_checksum_address(REVERSE_ADDRESS).lower()
                    ]:
                        print(f"🎯 Found transaction to Base Name Service contract!")
                        print(f"   Block: {block_num}")
                        print(f"   Tx Hash: {tx.hash.hex()}")
                        print(f"   To: {tx.to}")
                        event_found = True
                        break
                if event_found:
                    break
            except Exception as e:
                continue
        
        if not event_found:
            print("📭 No recent Base Name Service transactions found")
            
    except Exception as e:
        print(f"❌ Error checking recent events: {e}")

def main():
    print("🚀 Testing Base Name Service on Base Network")
    print("=" * 50)
    
    # Test connection
    if not test_base_connection():
        return
    
    print("\n📋 Testing contract addresses:")
    test_contract_addresses()
    
    # Test recent events
    test_recent_events()
    
    print("\n✅ Test completed!")

if __name__ == "__main__":
    main() 