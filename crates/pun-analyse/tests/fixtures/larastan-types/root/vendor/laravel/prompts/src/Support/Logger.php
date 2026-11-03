<?php

namespace Laravel\Prompts\Support;

class Logger
{
    /**
     * Create a new Logger instance.
     *
     * @param  resource|null  $socket
     */
    public function __construct(protected string $identifier, protected $socket = null)
    {}

    /**
     * The buffer for streaming text.
     */
    protected string $streamBuffer = '';

    /**
     * Log a line to the process log.
     */
    public function line(string $message): void
    {}

    /**
     * Append a chunk of text, accumulating on the current line(s).
     */
    public function partial(string $chunk): void
    {}

    /**
     * Commit the accumulated partial text and start fresh.
     */
    public function commitPartial(): void
    {}

    /**
     * Log a success message to the process log.
     */
    public function success(string $message): void
    {}

    /**
     * Log a warning message to the process log.
     */
    public function warning(string $message): void
    {}

    /**
     * Log an error message to the process log.
     */
    public function error(string $message): void
    {}

    /**
     * Update the label of the process log.
     */
    public function label(string $message): void
    {}

    /**
     * Update the sub-label of the process log. Pass an empty string to clear.
     */
    public function subLabel(string $message): void
    {}

    /**
     * Write a message to the socket.
     */
    protected function write(string $message, ?string $type = null): void
    {}

    /**
     * Prefix a message with the identifier and type.
     */
    protected function prefix(string $type, string $message): string
    {}
}
