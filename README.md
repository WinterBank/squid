<a href="https://peerlync.com">
	<img width="100%" src="./.assets/squid_banner.png" alt="Squid Banner" />
</a>

---

[![License](https://img.shields.io/badge/license-Apache-green.svg)](LICENSE)

Squid is a layer 1 blockchain.

### Miner Earnings
Over a sufficiently long time, the miner’s earnings will be the same regardless of difficulty, because:

- The protocol adjusts difficulty dynamically to ensure a fixed target time of one block per minute.
- Higher difficulty results in fewer rewarded hashes but larger rewards per hash.
- Lower difficulty results in more rewarded hashes but smaller rewards per hash.

#### Mathematical Explanation:
The miner’s earnings over time are proportional to:

**Earnings**: $P(\text{difficulty}) \cdot 1.524287^{\text{difficulty}}$

Where:
- $P(\text{difficulty})$ is the probability of finding a valid hash at a given difficulty.
- $1.524287^{\text{difficulty}}$ represents the reward for achieving that difficulty.

The dynamic adjustment ensures that $P(\text{difficulty})$ balances $1.524287^{\text{difficulty}}$, leading to the same average earnings over time.

---

### Visual Representation

Below are two charts:
1. **Reward vs. Difficulty:** Illustrates how rewards grow exponentially with difficulty.
2. **Expected Earnings vs. Difficulty:** Shows how expected earnings remain constant over time due to the balancing effect of difficulty adjustment.

![Miner Earnings Charts](./.assets/miner_earnings_charts.png)

- **Reward vs. Difficulty:** As difficulty increases, the reward for a valid hash grows exponentially ($1.524287^{\text{difficulty}}$).
- **Expected Earnings vs. Difficulty:** Despite the increase in reward with difficulty, the expected earnings remain constant due to the decreasing probability of finding a valid hash.

---

### Conclusion
The dynamic difficulty adjustment ensures miners earn the same over a sufficiently long time, regardless of the difficulty level.

### Maximum Supply
- The maximum supply of coins in the network is 2,305,843,009,213,693,951.
- This is the largest Mersenne prime that fits into a 64-bit integer.
- It can also be represented as $2^{\text{61}} - 1$.

## Getting Started

* [Guide - Integrate with the Aptos Blockchain](https://aptos.dev/guides/system-integrators-guide)
* [Tutorials](https://aptos.dev/tutorials)

## Contributing

You can learn more about contributing to the Aptos project by reading our [Contribution Guide](https://github.com/aptos-labs/aptos-core/blob/main/CONTRIBUTING.md) and by viewing our [Code of Conduct](https://github.com/aptos-labs/aptos-core/blob/main/CODE_OF_CONDUCT.md).

Aptos Core is licensed under [Apache 2.0](https://github.com/aptos-labs/aptos-core/blob/main/LICENSE).
