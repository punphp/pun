<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Mailer\Transport;

use Psr\EventDispatcher\EventDispatcherInterface;
use Psr\Log\LoggerInterface;
use Symfony\Component\Mailer\Exception\IncompleteDsnException;
use Symfony\Contracts\HttpClient\HttpClientInterface;

/**
 * @author Konstantin Myakshin <molodchick@gmail.com>
 */
abstract class AbstractTransportFactory implements TransportFactoryInterface
{
    public function __construct(
        protected ?EventDispatcherInterface $dispatcher = null,
        protected ?HttpClientInterface $client = null,
        protected ?LoggerInterface $logger = null,
    ) {}

    public function supports(Dsn $dsn): bool
    {}

    abstract protected function getSupportedSchemes(): array;

    protected function getUser(Dsn $dsn): string
    {}

    protected function getPassword(Dsn $dsn): string
    {}
}
