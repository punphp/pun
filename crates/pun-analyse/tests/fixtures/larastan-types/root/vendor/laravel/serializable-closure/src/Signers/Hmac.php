<?php

namespace Laravel\SerializableClosure\Signers;

use Laravel\SerializableClosure\Contracts\Signer;

class Hmac implements Signer
{
    /**
     * The secret key.
     *
     * @var string
     */
    protected $secret;

    /**
     * Creates a new signer instance.
     *
     * @param  string  $secret
     * @return void
     */
    public function __construct($secret)
    {}

    /**
     * Sign the given serializable.
     *
     * @param  string  $serialized
     * @return array
     */
    public function sign($serialized)
    {}

    /**
     * Verify the given signature.
     *
     * @param  array{serializable: string, hash: string}  $signature
     * @return bool
     */
    public function verify($signature)
    {}
}
