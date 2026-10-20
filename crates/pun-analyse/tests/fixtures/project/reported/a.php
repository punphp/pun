<?php

trait ReportedTrait
{
    public function t(): void
    {
        missing_in_trait();
    }
}

class ReportedUser
{
    use ReportedTrait;
}

missing_fn(); // @phpstan-ignore function.notFound
new ReportedMissing();
