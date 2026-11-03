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

use Monolog\Formatter\FormatterInterface;
use Monolog\ResettableInterface;
use Monolog\LogRecord;

/**
 * Forwards records to multiple handlers
 *
 * @author Lenar Lõhmus <lenar@city.ee>
 */
class GroupHandler extends Handler implements ProcessableHandlerInterface, ResettableInterface
{
    use ProcessableHandlerTrait;

    /** @var HandlerInterface[] */
    protected array $handlers;
    protected bool $bubble;

    /**
     * @param HandlerInterface[] $handlers Array of Handlers.
     * @param bool               $bubble   Whether the messages that are handled can bubble up the stack or not
     *
     * @throws \InvalidArgumentException if an unsupported handler is set
     */
    public function __construct(array $handlers, bool $bubble = true)
    {}

    /**
     * @inheritDoc
     */
    public function isHandling(LogRecord $record): bool
    {}

    /**
     * @inheritDoc
     */
    public function handle(LogRecord $record): bool
    {}

    /**
     * @inheritDoc
     */
    public function handleBatch(array $records): void
    {}

    public function reset(): void
    {}

    public function close(): void
    {}

    /**
     * @inheritDoc
     */
    public function setFormatter(FormatterInterface $formatter): HandlerInterface
    {}
}
