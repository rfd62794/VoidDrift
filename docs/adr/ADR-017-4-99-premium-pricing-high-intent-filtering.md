# ADR-017: $4.99 premium pricing with high-intent filtering
**Date:** May 2026
**Status:** Accepted

## Context
The game is approaching commercial release on itch.io. Pricing strategy is critical for revenue and player acquisition. The mobile idle game market typically uses:
- Free-to-play with microtransactions (aggressive monetization, pay-to-win concerns)
- $0.99-$2.99 premium (low barrier, but attracts low-intent players who may refund or leave negative reviews)
- $4.99-$9.99 premium (higher barrier, but attracts committed players who value the product)

Key considerations:
- The game is a niche survival sci-fi idle game with a specific target audience
- Development is solo (no team to support ongoing free-to-play maintenance)
- The game has a complete gameplay loop with 2-3 hours of content (not endless grind)
- itch.io audience is niche and quality-focused
- Refunds and negative reviews from low-intent players can damage reputation

## Decision
Set the price at $4.99 with high-intent filtering:
- **Price point**: $4.99 premium (one-time purchase, no microtransactions, no ads)
- **Filtering mechanism**: Higher price naturally filters for high-intent players who:
  - Read the description and understand the game's premise
  - Are willing to pay for a complete, polished experience
  - Are less likely to refund or leave negative reviews
  - Value the niche survival sci-fi theme
- **Value proposition**: Complete game, no DLC, no microtransactions, no ads, one purchase
- **Platform**: itch.io only (niche audience, quality-focused, supports premium pricing)

## Rationale
This pricing strategy provides:
- **Quality filtering**: $4.99 filters out low-intent players who might refund or leave negative reviews
- **Revenue adequacy**: $4.99 × expected volume provides meaningful revenue for solo development
- **Alignment with niche**: Sci-fi survival idle is a niche — $4.99 attracts the right audience
- ** itch.io fit**: itch.io audience expects quality and is willing to pay for it
- **No monetization complexity**: No microtransactions, no ads, no in-game purchases
- **Complete experience**: Players get the full game upfront, no hidden costs
- **Reputation protection**: High-intent players are more likely to leave positive reviews

## Consequences
- **Positive**: Filters for high-intent players who value the game's premise
- **Positive**: Provides meaningful revenue for solo development
- **Positive**: No ongoing monetization complexity (no microtransactions, no ads)
- **Positive**: Aligns with itch.io's quality-focused audience
- **Positive**: Players get complete experience upfront
- **Constraint**: Lower conversion rate compared to free-to-play or $0.99
- **Constraint**: Marketing must emphasize quality and completeness to justify price
- **Constraint**: Game must be polished and bug-free to justify premium price
- **Risk**: Small niche market may limit total volume
- **Mitigation**: itch.io's audience is niche and quality-focused, aligns with the game's theme
