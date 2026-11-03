<?php

namespace Orchestra\Testbench\Foundation\Console;

use Illuminate\Foundation\Console\VendorPublishCommand as Command;

use function Orchestra\Testbench\transform_realpath_to_relative;

/**
 * @codeCoverageIgnore
 */
class VendorPublishCommand extends Command
{
    /** {@inheritDoc} */
    #[\Override]
    protected function status($from, $to, $type)
    {}
}
