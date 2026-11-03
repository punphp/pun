<?php

namespace Egulias\EmailValidator\Result;

use Egulias\EmailValidator\Result\Reason\Reason;

class InvalidEmail implements Result
{
    /**
     * @var string
     */
    private string $token;

    /**
     * @var Reason
     */
    protected Reason $reason;

    public function __construct(Reason $reason, string $token)
    {}

    public function isValid(): bool
    {}

    public function isInvalid(): bool
    {}

    public function description(): string
    {}

    public function code(): int
    {}

    public function reason(): Reason
    {}
}
