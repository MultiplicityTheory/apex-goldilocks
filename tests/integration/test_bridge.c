// End-to-end integration test harness
#include <stdio.h>
#include <assert.h>
#include "../../bridge/goldilocks_atlas_bridge.c"

int main() {
    atlas_bridge_t bridge;
    initialize_atlas_bridge(&bridge, 1);
    assert(bridge.prime_gate == 2);
    printf("--- Integration Test PASSED ---\n");
    return 0;
}
