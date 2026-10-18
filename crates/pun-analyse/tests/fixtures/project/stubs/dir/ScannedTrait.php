<?php

namespace Main;

trait ScannedTrait
{
    public function make(): void
    {
        new MissingFromScannedTrait();
        \missing_from_scanned_trait();
    }
}
