# hello-sat

### How to start API?
Download or clone repo, then run command `cargo run`. API is available at `http://localhost:8000`.
### What endpoints are available?
There are available two endpoints with given query strings:

1. `/calculateDisselUsageForDistance?distance=&yearOfProduction=&fuelUsagePer100KM=`

For example: `http://localhost:8000/calculateDisselUsageForDistance?distance=200&yearOfProduction=2010&fuelUsagePer100KM=5.2`

2. `/probabilityOfUnitInjectorFail?VIN=`

For example: `http://localhost:8000/probabilityOfUnitInjectorFail?VIN=234fds223rew`

