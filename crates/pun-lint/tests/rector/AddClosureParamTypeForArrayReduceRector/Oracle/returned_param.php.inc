<?php

final class TrustAll
{
    /**
     * @param  array  $trustedIps
     */
    public function append(array $trustedIps)
    {
        return array_reduce($trustedIps, function ($ips, $trustedIp) {
            $ips[] = $trustedIp;

            return $ips;
        }, []);
    }

    /**
     * @param  list<string>  $trustedIps
     */
    public function keyed(array $trustedIps)
    {
        return array_reduce($trustedIps, function ($ips, $trustedIp) {
            $ips[$trustedIp] = true;

            return $ips;
        }, []);
    }

    public function returnsCarry(array $items)
    {
        return array_reduce($items, function ($carry, $item) {
            return $carry;
        }, []);
    }
}
-----
<?php

final class TrustAll
{
    /**
     * @param  array  $trustedIps
     */
    public function append(array $trustedIps)
    {
        return array_reduce($trustedIps, function ($ips, $trustedIp) {
            $ips[] = $trustedIp;

            return $ips;
        }, []);
    }

    /**
     * @param  list<string>  $trustedIps
     */
    public function keyed(array $trustedIps)
    {
        return array_reduce($trustedIps, function ($ips, string $trustedIp) {
            $ips[$trustedIp] = true;

            return $ips;
        }, []);
    }

    public function returnsCarry(array $items)
    {
        return array_reduce($items, function ($carry, $item) {
            return $carry;
        }, []);
    }
}
