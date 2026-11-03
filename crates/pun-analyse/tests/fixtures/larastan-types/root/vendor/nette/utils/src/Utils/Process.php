<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Utils;

use Nette;
use function is_resource, is_string, strlen;
use const PHP_VERSION_ID;


/**
 * Represents a process, which can be started and controlled (reading output, writing input, waiting for completion).
 */
final class Process
{
	private const PollInterval = 10_000;
	private const DefaultTimeout = 60;
	private const StdIn = 0;
	private const StdOut = 1;
	private const StdErr = 2;

	/** @var resource */
	private mixed $process;

	/** @var array<string, mixed>  result of proc_get_status() */
	private array $status = ['running' => true];

	/** @var resource */
	private mixed $inputPipe;

	/** @var array<int, resource|string[]>  pipe resources, or ['file', path, mode] descriptors for discarded output */
	private array $outputPipes = [];

	/** @var string[] */
	private array $outputBuffers = [];

	/** @var int[] Number of bytes already read from buffers. */
	private array $outputBufferOffsets = [];

	/** @var array<int, true> Output IDs whose target resource was supplied by the caller and must not be closed here. */
	private array $callerOutputs = [];

	/** @var array<int, true> Output IDs whose pipe is backed by a temporary file (Windows < 8.5 workaround). */
	private array $fileBackedOutputs = [];
	private float $startTime;
	private bool $detached = false;


	/**
	 * Starts an executable with given arguments. Because the arguments are passed as an array, the shell is
	 * never involved, so they need no escaping and there is no risk of shell injection.
	 * @param  string         $executable Path to the executable binary.
	 * @param  list<string>   $arguments  Arguments passed to the executable.
	 * @param  string[]|null  $env        Environment variables or null to use the same environment as the current process.
	 * @param  array<string, mixed>  $options  Options for proc_open(); on Windows the executable is launched directly, without cmd.exe.
	 * @param  mixed          $stdin      Input: string, a readable resource (its content is copied to STDIN),
	 *                                    a Process (its STDOUT is piped in), or null (STDIN stays open for writeStdInput()).
	 * @param  mixed          $stdout     Output target: string filename, a writable resource backed by a real OS file descriptor
	 *                                    (not php://memory etc.), false to discard, or null to capture into memory.
	 * @param  mixed          $stderr     Error output target (same options as $stdout).
	 * @param  string|null    $directory  Working directory.
	 * @param  float|null     $timeout    Time limit in seconds, checked while waiting for or reading the process; null disables it.
	 */
	public static function runExecutable(
		string $executable,
		array $arguments = [],
		?array $env = null,
		array $options = [],
		mixed $stdin = '',
		mixed $stdout = null,
		mixed $stderr = null,
		?string $directory = null,
		?float $timeout = self::DefaultTimeout,
	): self
	{}


	/**
	 * Starts a process from a command string interpreted by the shell (/bin/sh on POSIX, cmd.exe on Windows).
	 * Because the shell parses the string, NEVER pass unescaped user input here - use runExecutable() for that.
	 * @param  string         $command    Shell command to run.
	 * @param  string[]|null  $env        Environment variables or null to use the same environment as the current process.
	 * @param  array<string, mixed>  $options  Options for proc_open(), e.g. ['bypass_shell' => true] on Windows to skip cmd.exe.
	 * @param  mixed          $stdin      Input: string, a readable resource (its content is copied to STDIN),
	 *                                    a Process (its STDOUT is piped in), or null (STDIN stays open for writeStdInput()).
	 * @param  mixed          $stdout     Output target: string filename, a writable resource backed by a real OS file descriptor
	 *                                    (not php://memory etc.), false to discard, or null to capture into memory.
	 * @param  mixed          $stderr     Error output target (same options as $stdout).
	 * @param  string|null    $directory  Working directory.
	 * @param  float|null     $timeout    Time limit in seconds, checked while waiting for or reading the process; null disables it.
	 */
	public static function runCommand(
		string $command,
		?array $env = null,
		array $options = [],
		mixed $stdin = '',
		mixed $stdout = null,
		mixed $stderr = null,
		?string $directory = null,
		?float $timeout = self::DefaultTimeout,
	): self
	{}


	/**
	 * @param  list<string>|string  $command
	 * @param  array<string, string>|null  $env
	 * @param  array<string, mixed>  $options
	 */
	private function __construct(
		string|array $command,
		?array $env,
		array $options,
		?string $directory,
		mixed $stdin,
		mixed $stdout,
		mixed $stderr,
		private readonly ?float $timeout,
	) {}


	public function __destruct()
	{}


	/**
	 * Detaches the process: it keeps running in the background and is not terminated when the object
	 * is destroyed. STDIN and output pipes are closed, so the output must not be captured in memory
	 * (pass a file name, a resource or false as $stdout/$stderr). Only the destructor behavior changes:
	 * wait() and getExitCode() still block ($timeout still applies) and terminate() still kills it.
	 * On POSIX, a detached child that exits early stays a zombie until the script ends.
	 */
	public function detach(): void
	{}


	/**
	 * Checks if the process is currently running.
	 */
	public function isRunning(): bool
	{}


	/**
	 * Finishes the process by waiting for its completion. While waiting, the captured output is read
	 * continuously and kept in memory; an optional callback is invoked with each new output/error chunk.
	 *
	 * @param  (\Closure(string, string): void)|null  $callback
	 */
	public function wait(?\Closure $callback = null): void
	{}


	/**
	 * Reads any new data from the captured pipes into the buffers, so a process producing more output
	 * than the OS pipe buffer holds does not block. With $final (process already finished) reads to EOF.
	 */
	private function drainPipes(bool $final = false): void
	{}


	/**
	 * Terminates the running process if it is still running.
	 */
	public function terminate(): void
	{}


	/**
	 * Returns the process exit code. If the process is still running, waits until it finishes.
	 */
	public function getExitCode(): int
	{}


	/**
	 * Returns true if the process terminated with exit code 0.
	 */
	public function isSuccess(): bool
	{}


	/**
	 * Waits for the process to finish and throws ProcessFailedException if exit code is not zero.
	 */
	public function ensureSuccess(): void
	{}


	/**
	 * Returns the PID of the running process, or null if it is not running.
	 */
	public function getPid(): ?int
	{}


	/**
	 * Waits for the process to finish and returns everything it wrote to STDOUT.
	 */
	public function getStdOutput(): string
	{}


	/**
	 * Waits for the process to finish and returns everything it wrote to STDERR.
	 */
	public function getStdError(): string
	{}


	/**
	 * Returns the STDOUT data produced since the previous consumeStdOutput() call.
	 * To read everything incrementally, poll `while ($p->isRunning())` calling this, then call it once more
	 * after the loop; that last call returns whatever the process wrote just before the loop noticed it had exited.
	 */
	public function consumeStdOutput(): string
	{}


	/**
	 * Returns the STDERR data produced since the previous consumeStdError() call. See consumeStdOutput().
	 */
	public function consumeStdError(): string
	{}


	/**
	 * Returns newly available data from the specified buffer and advances the read pointer.
	 */
	private function consumeBuffer(int $id): string
	{}


	/**
	 * Returns the buffered data not returned yet and advances the read pointer.
	 */
	private function extractNewData(int $id): string
	{}


	/**
	 * Writes data into the process' STDIN. If STDIN is closed, throws exception.
	 */
	public function writeStdInput(string $string): void
	{}


	/**
	 * Writes the whole string to STDIN, handling partial writes. Stops (and lets fwrite() warn) on a broken pipe,
	 * i.e. when the process stopped reading its STDIN.
	 */
	private function writeToPipe(string $string): void
	{}


	/**
	 * Closes the STDIN pipe, indicating no more data will be sent.
	 */
	public function closeStdInput(): void
	{}


	/**
	 * If a callback is given, invokes it with the output/error produced since the previous call.
	 * @param  (\Closure(string, string): void)|null  $callback
	 */
	private function dispatchCallback(?\Closure $callback): void
	{}


	/**
	 * Checks if the timeout has expired. If yes, terminates the process.
	 */
	private function enforceTimeout(): void
	{}


	/**
	 * Reads any new data from the specified pipe and appends it to the buffer. Does nothing if the output
	 * is not captured or the pipe is already closed (or handed over to another process).
	 */
	private function readFromPipe(int $id, bool $final = false): void
	{}


	/**
	 * Sends the initial input to the process: writes and closes a string or stream input,
	 * or leaves STDIN open when input is null (until closeStdInput()) or another Process (fed by that process).
	 * The input type was already validated by createInputDescriptor().
	 *
	 * Note: a string or stream input is written upfront, so if it is large and the process does not read it
	 * while filling its own output, this can block; in that case pass null and feed STDIN via writeStdInput().
	 */
	private function writeInitialInput(mixed $input): void
	{}


	/**
	 * Validates the input and determines the STDIN descriptor based on its type.
	 */
	private function createInputDescriptor(mixed $input): mixed
	{}


	/**
	 * Determines the descriptor for STDOUT or STDERR based on the specified output target.
	 */
	private function createOutputDescriptor(int $id, mixed $output): mixed
	{}


	/**
	 * Closes all pipes and the process resource.
	 */
	private function close(): void
	{}


	/**
	 * Closes the output pipes that this class opened; resources supplied by the caller are left untouched.
	 * (The temporary file backing captured output on Windows < 8.5 is removed by fclose() itself.)
	 */
	private function closeOutputPipes(): void
	{}
}
