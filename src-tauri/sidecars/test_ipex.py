#!/usr/bin/env python3
"""
Test script to verify Intel Extension for PyTorch installation
"""

import sys
import torch
import intel_extension_for_pytorch as ipex
from intel_extension_for_pytorch import __version__ as ipex_version

def test_environment():
    print(f"Python version: {sys.version}")
    print(f"IPEX-LLM version: {ipex_version}")
    print(f"PyTorch version: {torch.__version__}")
    
    # Check if GPU is available
    if torch.cuda.is_available():
        print("✓ CUDA is available")
        print(f"Number of GPUs: {torch.cuda.device_count()}")
        for i in range(torch.cuda.device_count()):
            print(f"GPU {i}: {torch.cuda.get_device_name(i)}")
    elif torch.xpu.is_available():
        print("✓ XPU (Intel Arc GPU) is available")
        print(f"Number of XPU devices: {torch.xpu.device_count()}")
        for i in range(torch.xpu.device_count()):
            print(f"XPU {i}: {torch.xpu.get_device_name(i)}")
    else:
        print("⚠️ No GPU acceleration available (using CPU)")
    
    return True

def test_basic_inference():
    try:
        # Create a simple tensor and perform a basic operation
        device = torch.device("xpu" if torch.xpu.is_available() else "cpu")
        print(f"\nTesting on device: {device}")
        
        x = torch.tensor([1, 2, 3], device=device)
        y = torch.tensor([4, 5, 6], device=device)
        result = x + y
        
        print(f"✓ Basic tensor operation successful: {result}")
        return True
        
    except Exception as e:
        print(f"⚠️ Error during basic inference: {str(e)}")
        return False

def main():
    print("=== IPEX-LLM Environment Test ===")
    
    # Test environment
    if not test_environment():
        return 1
    
    # Test basic inference
    if not test_basic_inference():
        return 2
    
    print("\n=== All tests passed! ===")
    return 0

if __name__ == "__main__":
    sys.exit(main())
