<?php

namespace Illuminate\Broadcasting\Mercure;

use InvalidArgumentException;
use Jose\Component\Core\AlgorithmManager;
use Jose\Component\Core\JWK;
use Jose\Component\Core\Util\Base64UrlSafe;
use Jose\Component\Encryption\Algorithm\ContentEncryption\A256GCM;
use Jose\Component\Encryption\Algorithm\KeyEncryption\Dir;
use Jose\Component\Encryption\JWEBuilder;
use Jose\Component\Encryption\Serializer\CompactSerializer;
use LogicException;
use SensitiveParameter;

/**
 * Encrypts Mercure updates end to end, so the hub never sees their content.
 *
 * @author Kévin Dunglas <kevin@dunglas.dev>
 */
class ChannelEncrypter
{
    /**
     * The JWE builder, built on first use and reused across updates.
     *
     * @var \Jose\Component\Encryption\JWEBuilder|null
     */
    protected $jweBuilder;

    /**
     * The compact JWE serializer.
     *
     * @var \Jose\Component\Encryption\Serializer\CompactSerializer|null
     */
    protected $serializer;

    /**
     * Create a new Mercure channel encrypter.
     *
     * @param  string  $key
     */
    public function __construct(#[SensitiveParameter] protected string $key)
    {}

    /**
     * Encrypt the given plaintext for the given channel, as a compact JWE.
     *
     * @param  string  $plaintext
     * @param  string  $channel
     * @return string
     */
    public function encrypt($plaintext, $channel)
    {}

    /**
     * Derive the AES-256-GCM key of the given channel.
     *
     * @param  string  $channel
     * @return string
     */
    public function channelKey($channel)
    {}

    /**
     * Build the JSON Web Key sharing the given channel's key with an authorized subscriber.
     *
     * @param  string  $channel
     * @return array{kty: string, k: string, alg: string, use: string}
     */
    public function channelJwk($channel)
    {}
}
