<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Util\PHP;

use function assert;
use function fclose;
use function feof;
use function fwrite;
use function is_resource;
use function proc_close;
use function rewind;
use function stream_get_contents;
use function stream_set_blocking;
use function unlink;

/**
 * Handle for a worker process that has been spawned by the JobRunner.
 *
 * Spawning (proc_open) and reaping (proc_close) a process are separated so
 * that a single thread of control can drive more than one process at a time:
 * a caller may start several jobs, multiplex their output streams with
 * stream_select(), feed each ready stream to consume(), and finally reap each
 * process with wait(). The synchronous JobRunner::run() is the degenerate case
 * of starting one job and immediately waiting for it.
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class RunningJob
{
    /**
     * @var resource
     */
    private $process;

    /**
     * @var ?resource
     */
    private $stdin;

    /**
     * @var ?resource
     */
    private $stdout;

    /**
     * @var ?resource
     */
    private $stderr;

    /**
     * @var ?resource
     */
    private $mergedOutputStream;
    private ?string $temporaryFile;
    private string $stdoutBuffer = '';
    private string $stderrBuffer = '';
    private ?Result $result      = null;

    /**
     * @param resource             $process
     * @param array<int, resource> $pipes
     * @param ?resource            $mergedOutputStream
     */
    public function __construct(mixed $process, array $pipes, mixed $mergedOutputStream, ?string $temporaryFile)
    {}

    /**
     * Write to the standard input of the worker process.
     */
    public function write(string $bytes): void
    {}

    /**
     * Signal end-of-input to the worker process by closing its standard input.
     */
    public function closeStdin(): void
    {}

    /**
     * @return ?resource
     */
    public function stdout(): mixed
    {}

    /**
     * The output streams that are still open and can be passed to
     * stream_select() to wait for the worker process to produce output.
     *
     * @return list<resource>
     */
    public function readableStreams(): array
    {}

    /**
     * Read whatever output is currently available without blocking. Intended
     * to be called after stream_select() has reported one of the streams
     * returned by readableStreams() as ready.
     */
    public function consume(): void
    {}

    /**
     * Block until the worker process has terminated, then reap it and return
     * its accumulated output.
     */
    public function wait(): Result
    {}

    /**
     * @param ?resource $stream
     */
    private function readAvailable(mixed &$stream): string
    {}

    /**
     * @param ?resource $stream
     */
    private function drain(mixed $stream): string
    {}
}
