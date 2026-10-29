<?php

namespace Probe;

trait ParsesServer
{
    /**
     * @return array{0: string|null, 1: int|null}
     */
    protected function server($server)
    {
        return [$server, null];
    }

    protected function destructured($options)
    {
        [$host, $port] = $this->server($options['server'] ?? null);

        return array_filter([
            'driver' => 'sqlsrv',
            'host' => $host,
            'port' => $port,
        ], fn ($value) => ! is_null($value));
    }

    protected function literal()
    {
        [$host, $port] = ['a', 'b'];

        return array_filter([$host, $port], fn ($value) => ! is_null($value));
    }

    protected function rewrapped($key)
    {
        $files = $this->server($key);
        $files = [$files[0]];

        return array_filter($files, fn ($file) => $file !== null);
    }
}

class Plain
{
    /**
     * @return array{0: string|null, 1: int|null}
     */
    protected function server($server)
    {
        return [$server, null];
    }

    protected function destructured($options)
    {
        [$host, $port] = $this->server($options['server'] ?? null);

        return array_filter([
            'driver' => 'sqlsrv',
            'host' => $host,
            'port' => $port,
        ], fn ($value) => ! is_null($value));
    }
}
-----
<?php

namespace Probe;

trait ParsesServer
{
    /**
     * @return array{0: string|null, 1: int|null}
     */
    protected function server($server)
    {
        return [$server, null];
    }

    protected function destructured($options)
    {
        [$host, $port] = $this->server($options['server'] ?? null);

        return array_filter([
            'driver' => 'sqlsrv',
            'host' => $host,
            'port' => $port,
        ], fn ($value) => ! is_null($value));
    }

    protected function literal()
    {
        [$host, $port] = ['a', 'b'];

        return array_filter([$host, $port], fn (string $value) => ! is_null($value));
    }

    protected function rewrapped($key)
    {
        $files = $this->server($key);
        $files = [$files[0]];

        return array_filter($files, fn ($file) => $file !== null);
    }
}

class Plain
{
    /**
     * @return array{0: string|null, 1: int|null}
     */
    protected function server($server)
    {
        return [$server, null];
    }

    protected function destructured($options)
    {
        [$host, $port] = $this->server($options['server'] ?? null);

        return array_filter([
            'driver' => 'sqlsrv',
            'host' => $host,
            'port' => $port,
        ], fn (string|int|null $value) => ! is_null($value));
    }
}
