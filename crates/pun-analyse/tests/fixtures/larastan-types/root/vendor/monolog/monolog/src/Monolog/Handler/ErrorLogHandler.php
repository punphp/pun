<?php declare(strict_types=1);

/*
 * This file is part of the Monolog package.
 *
 * (c) Jordi Boggiano <j.boggiano@seld.be>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Monolog\Handler;

use Monolog\Formatter\LineFormatter;
use Monolog\Formatter\FormatterInterface;
use Monolog\Level;
use Monolog\LogRecord;

/**
 * Stores to PHP error_log() handler.
 *
 * @author Elan Ruusamäe <glen@delfi.ee>
 */
class ErrorLogHandler extends AbstractProcessingHandler
{
    public const OPERATING_SYSTEM = 0;
    public const SAPI = 4;

    /** @var 0|4 */
    protected int $messageType;
    protected bool $expandNewlines;

    /**
     * @param 0|4 $messageType    Says where the error should go.
     * @param bool $expandNewlines If set to true, newlines in the message will be expanded to be take multiple log entries
     *
     * @throws \InvalidArgumentException If an unsupported message type is set
     */
    public function __construct(int $messageType = self::OPERATING_SYSTEM, int|string|Level $level = Level::Debug, bool $bubble = true, bool $expandNewlines = false)
    {}

    /**
     * @return int[] With all available types
     */
    public static function getAvailableTypes(): array
    {}

    /**
     * @inheritDoc
     */
    protected function getDefaultFormatter(): FormatterInterface
    {}

    /**
     * @inheritDoc
     */
    protected function write(LogRecord $record): void
    {}
}
