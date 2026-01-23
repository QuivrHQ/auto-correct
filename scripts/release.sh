#!/bin/bash
set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if version argument is provided
if [ -z "$1" ]; then
  echo -e "${RED}Error: Version argument required${NC}"
  echo "Usage: pnpm release <version>"
  echo "Example: pnpm release 1.3.0"
  exit 1
fi

VERSION=$1

# Validate version format (semver)
if ! [[ $VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  echo -e "${RED}Error: Invalid version format. Use semver (e.g., 1.2.3)${NC}"
  exit 1
fi

echo -e "${YELLOW}Starting release v${VERSION}...${NC}"

# Check for uncommitted changes
if [ -n "$(git status --porcelain)" ]; then
  echo -e "${RED}Error: Working directory not clean. Commit or stash changes first.${NC}"
  exit 1
fi

# Update version in package.json
echo -e "${GREEN}Updating package.json...${NC}"
sed -i '' "s/\"version\": \"[^\"]*\"/\"version\": \"${VERSION}\"/" package.json

# Update version in manifest.json
echo -e "${GREEN}Updating manifest.json...${NC}"
sed -i '' "s/\"version\": \"[^\"]*\"/\"version\": \"${VERSION}\"/" public/manifest.json

# Detect package manager
if [ -f "pnpm-lock.yaml" ]; then
  PM="pnpm"
  LOCK_FILE="pnpm-lock.yaml"
elif [ -f "package-lock.json" ]; then
  PM="npm"
  LOCK_FILE="package-lock.json"
else
  PM="npm"
  LOCK_FILE=""
fi

# Update dependencies
echo -e "${GREEN}Updating dependencies with ${PM}...${NC}"
$PM update

# Build
echo -e "${GREEN}Building...${NC}"
$PM run build

# Stage and commit
echo -e "${GREEN}Creating commit...${NC}"
git add package.json public/manifest.json
[ -n "$LOCK_FILE" ] && git add "$LOCK_FILE"
git commit -m "chore: release v${VERSION}"

# Create tag
echo -e "${GREEN}Creating tag v${VERSION}...${NC}"
git tag -a "v${VERSION}" -m "Release v${VERSION}"

# Push
echo -e "${GREEN}Pushing to origin...${NC}"
git push origin main
git push origin "v${VERSION}"

echo -e "${GREEN}✓ Release v${VERSION} completed!${NC}"
