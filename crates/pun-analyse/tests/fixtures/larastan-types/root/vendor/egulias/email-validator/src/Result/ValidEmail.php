<?php

namespace Egulias\EmailValidator\Result;

class ValidEmail implements Result
{
    public function isValid(): bool
    {}

    public function isInvalid(): bool
    {}

    public function description(): string
    {}

    public function code(): int
    {}

}
