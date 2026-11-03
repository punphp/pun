<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Framework\TestCase;

use function fclose;
use function ini_get;
use function ini_set;
use function is_writable;
use function stream_get_contents;
use function stream_get_meta_data;
use function tmpfile;
use PHPUnit\Framework\Assert;
use PHPUnit\Framework\ErrorLogNotWritableException;
use PHPUnit\Runner\ShutdownHandler;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class ErrorLogCapture
{
    private bool $expectErrorLog = false;

    /**
     * @var false|resource
     */
    private mixed $captureHandle            = false;
    private false|string $previousLogTarget = false;

    public function expect(): void
    {}

    /**
     * @throws ErrorLogNotWritableException
     */
    public function verify(): void
    {}

    public function stop(): void
    {}

    private function start(): void
    {}
}
