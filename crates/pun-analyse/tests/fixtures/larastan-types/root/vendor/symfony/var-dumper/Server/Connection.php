<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\VarDumper\Server;

use Symfony\Component\VarDumper\Cloner\Data;
use Symfony\Component\VarDumper\Dumper\ContextProvider\ContextProviderInterface;

/**
 * Forwards serialized Data clones to a server.
 *
 * @author Maxime Steinhausser <maxime.steinhausser@gmail.com>
 */
class Connection
{
    private string $host;

    /**
     * @var resource|null
     */
    private $socket;

    /**
     * @param string                     $host             The server host
     * @param ContextProviderInterface[] $contextProviders Context providers indexed by context name
     */
    public function __construct(
        string $host,
        private array $contextProviders = [],
    ) {}

    public function getContextProviders(): array
    {}

    public function write(Data $data): bool
    {}

    /**
     * @return resource|null
     */
    private function createSocket()
    {}
}
