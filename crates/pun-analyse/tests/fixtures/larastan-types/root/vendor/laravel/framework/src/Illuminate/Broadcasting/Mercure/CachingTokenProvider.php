<?php

namespace Illuminate\Broadcasting\Mercure;

use Symfony\Component\Mercure\Jwt\TokenProviderInterface;

/**
 * @author Kévin Dunglas <kevin@dunglas.dev>
 */
class CachingTokenProvider implements TokenProviderInterface
{
    /**
     * The cached token.
     *
     * @var string|null
     */
    protected $jwt;

    /**
     * The Unix timestamp after which the cached token must be re-minted.
     *
     * @var int
     */
    protected $refreshAfter = 0;

    /**
     * Create a new caching token provider instance.
     *
     * @param  \Symfony\Component\Mercure\Jwt\TokenProviderInterface  $provider
     * @param  int  $clockSkew
     */
    public function __construct(
        protected TokenProviderInterface $provider,
        protected int $clockSkew = 30,
    ) {}

    /**
     * Get the JWT, minting a fresh one when the cached token nears expiry.
     */
    public function getJwt(): string
    {}

    /**
     * Extract the "exp" claim from the given JWT, if any.
     *
     * @param  string  $jwt
     * @return int|null
     */
    protected function expiresAt($jwt)
    {}
}
