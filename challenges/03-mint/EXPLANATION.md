## Vulnerability

Vulnerability is presented when contract instantiator instantiates contract with MINT_PER_USER that is larger than MAX_LIMIT for taking from map.
What happens is, when query_mint_per_user is executed, it will return only MAX_LIMIT which is less than number of mints, and user can mint unlimited amount of tokens.

## Solution

1. Not use limits when querying the map
2. Admin should instantiate with correct mint_per_user.