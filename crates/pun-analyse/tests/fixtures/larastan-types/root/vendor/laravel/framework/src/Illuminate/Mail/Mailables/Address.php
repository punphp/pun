<?php

namespace Illuminate\Mail\Mailables;

use InvalidArgumentException;

class Address
{
    /**
     * The recipient's email address.
     *
     * @var string
     */
    public $address;

    /**
     * The recipient's name.
     *
     * @var string|null
     */
    public $name;

    /**
     * Create a new address instance.
     *
     * @param  string  $address
     * @param  string|null  $name
     */
    public function __construct(string $address, ?string $name = null)
    {}
}
