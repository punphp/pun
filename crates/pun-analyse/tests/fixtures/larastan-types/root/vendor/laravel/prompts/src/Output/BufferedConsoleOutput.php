<?php

namespace Laravel\Prompts\Output;

class BufferedConsoleOutput extends ConsoleOutput
{
    /**
     * The output buffer.
     */
    protected string $buffer = '';

    /**
     * Empties the buffer and returns its content.
     */
    public function fetch(): string
    {}

    /**
     * Return the content of the buffer.
     */
    public function content(): string
    {}

    /**
     * Write to the output buffer.
     */
    protected function doWrite(string $message, bool $newline): void
    {}

    /**
     * Write output directly, bypassing newline capture.
     */
    public function writeDirectly(string $message): void
    {}
}
