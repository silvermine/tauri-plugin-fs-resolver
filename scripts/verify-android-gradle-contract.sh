#!/usr/bin/env bash
# Guards the plugin Android Gradle contract and README toolchain floors (no Android SDK).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
GRADLE_KTS="${ROOT}/android/build.gradle.kts"
GRADLE_PROPS="${ROOT}/android/gradle.properties"
CONSUMER_RULES="${ROOT}/android/consumer-rules.pro"
README="${ROOT}/README.md"

readonly EXPECTED_COMPILE_SDK=37
readonly README_GRADLE_FLOOR="9.3.1"
readonly README_COMPILE_SDK="compileSdk 37"

fail() {
  echo "verify-android-gradle-contract: $*" >&2
  exit 1
}

[[ -f "$GRADLE_KTS" ]] || fail "missing ${GRADLE_KTS}"
[[ -f "$GRADLE_PROPS" ]] || fail "missing ${GRADLE_PROPS}"
[[ -f "$CONSUMER_RULES" ]] || fail "missing ${CONSUMER_RULES}"

grep -q 'android.builtInKotlin=false' "$GRADLE_PROPS" \
  || fail "expected android.builtInKotlin=false in android/gradle.properties"
grep -q 'android.newDsl=false' "$GRADLE_PROPS" \
  || fail "expected android.newDsl=false in android/gradle.properties"
[[ -f "$README" ]] || fail "missing ${README}"

grep -q "compileSdk = ${EXPECTED_COMPILE_SDK}" "$GRADLE_KTS" \
  || fail "expected compileSdk = ${EXPECTED_COMPILE_SDK} in android/build.gradle.kts"

grep -q 'consumerProguardFiles("consumer-rules.pro")' "$GRADLE_KTS" \
  || fail 'expected consumerProguardFiles("consumer-rules.pro") in android/build.gradle.kts'

grep -q 'compilerOptions' "$GRADLE_KTS" \
  || fail "expected kotlin compilerOptions DSL in android/build.gradle.kts"

if grep -q 'kotlinOptions' "$GRADLE_KTS"; then
  fail "android/build.gradle.kts must not use kotlinOptions (use compilerOptions)"
fi

grep -q "$README_GRADLE_FLOOR" "$README" \
  || fail "README must document Gradle floor ${README_GRADLE_FLOOR}"

grep -q "$README_COMPILE_SDK" "$README" \
  || fail "README must document ${README_COMPILE_SDK}"

echo "verify-android-gradle-contract: ok"
