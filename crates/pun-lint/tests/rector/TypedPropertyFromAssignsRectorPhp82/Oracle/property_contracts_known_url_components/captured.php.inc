<?php

namespace QualityPropertyKnownUrlComponents;

final class KnownUrlComponents
{
    private $host;
    private $port;
    private $invalid;
    private $callback;

    public function __construct(string $url, callable $callback)
    {
        $this->host = parse_url($url, PHP_URL_HOST);
        $this->port = parse_url($url, PHP_URL_PORT);
        $this->invalid = parse_url($url, 999);
        $this->callback = $callback($url);
    }
}
-----
<?php

namespace QualityPropertyKnownUrlComponents;

final class KnownUrlComponents
{
    private string|bool|null $host = null;
    private int|bool|null $port = null;
    private bool $invalid;
    private $callback;

    public function __construct(string $url, callable $callback)
    {
        $this->host = parse_url($url, PHP_URL_HOST);
        $this->port = parse_url($url, PHP_URL_PORT);
        $this->invalid = parse_url($url, 999);
        $this->callback = $callback($url);
    }
}
