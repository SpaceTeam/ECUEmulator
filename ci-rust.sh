#!/usr/bin/env bash

# CI script for ECUEmulator Rust project
# This script runs all the checks that are performed in the GitHub Actions workflow

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print step headers
print_step() {
    echo -e "\n${YELLOW}==== $1 ====${NC}\n"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

# Get the script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_DIR="$SCRIPT_DIR"

# Export environment variable
export CARGO_TERM_COLOR=always

# Function to run a specific step or all steps
run_step() {
    local step=$1
    
    case $step in
        build)
            print_step "Build"
            cd "$PROJECT_DIR"
            cargo build --verbose || { print_error "Build failed"; return 1; }
            print_success "Build completed"
            ;;
        
        test)
            print_step "Run tests"
            cd "$PROJECT_DIR"
            cargo test --verbose || { print_error "Tests failed"; return 1; }
            print_success "Tests passed"
            ;;
        
        fmt)
            print_step "Check formatting"
            cd "$PROJECT_DIR"
            cargo fmt --all -- --check || { print_error "Formatting check failed"; return 1; }
            print_success "Formatting check passed"
            ;;

        fmt-fix)
            print_step "Fix formatting"
            cd "$PROJECT_DIR"
            cargo fmt --all || { print_error "Formatting fix failed"; return 1; }
            print_success "Formatting fixed"
            ;;
        
        clippy)
            print_step "Run clippy"
            cd "$PROJECT_DIR"
            cargo clippy --all-targets --all-features -- -D warnings || { print_error "Clippy check failed"; return 1; }
            print_success "Clippy check passed"
            ;;

        clippy-fix)
            print_step "Run clippy with fix"
            cd "$PROJECT_DIR"
            cargo clippy --all-targets --all-features --fix -- -D warnings || { print_error "Clippy fix failed"; return 1; }
            print_success "Clippy fix passed"
            ;;

        all)
            run_step build || return 1
            run_step test || return 1
            run_step fmt || return 1
            run_step clippy || return 1
            ;;
        
        *)
            echo "Usage: $0 [build|test|fmt|fmt-fix|clippy|clippy-fix|all]"
            echo ""
            echo "Steps:"
            echo "  build      - Build the project"
            echo "  test       - Run tests"
            echo "  fmt        - Check formatting"
            echo "  fmt-fix    - Fix formatting"
            echo "  clippy     - Run clippy linter"
            echo "  clippy-fix - Run clippy with automatic fixes"
            echo "  all        - Run all steps (default)"
            exit 1
            ;;
    esac
}

# Main execution
STEP=${1:-all}
echo "Running Rust CI checks..."
echo "Step: $STEP"

if run_step "$STEP"; then
    echo -e "\n${GREEN}All checks passed successfully!${NC}"
    exit 0
else
    echo -e "\n${RED}Checks failed!${NC}"
    exit 1
fi
