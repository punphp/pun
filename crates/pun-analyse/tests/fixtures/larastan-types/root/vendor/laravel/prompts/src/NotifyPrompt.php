<?php

namespace Laravel\Prompts;

use Symfony\Component\Process\ExecutableFinder;
use Symfony\Component\Process\Process;

class NotifyPrompt extends Prompt
{
    /**
     * Create a new NotifyPrompt instance.
     */
    public function __construct(
        public string $title,
        public string $body = '',
        public string $subtitle = '',
        public string $sound = '',
        public string $icon = '',
    ) {}

    /**
     * Send the notification.
     */
    public function prompt(): bool
    {}

    /**
     * Send a notification on macOS using osascript.
     */
    protected function sendMacOS(): bool
    {}

    /**
     * Send a notification on Linux, trying available notifiers.
     */
    protected function sendLinux(): bool
    {}

    /**
     * Send a notification using notify-send.
     */
    protected function sendLinuxNotifySend(): bool
    {}

    /**
     * Send a notification using kdialog.
     */
    protected function sendLinuxKDialog(): bool
    {}

    /**
     * Execute a command and return whether it was successful.
     *
     * @param  array<int, string>  $command
     */
    protected function execute(array $command): bool
    {}

    /**
     * Escape a string for use in AppleScript.
     */
    protected function escapeAppleScript(string $value): string
    {}

    /**
     * Send the notification.
     */
    public function display(): void
    {}

    /**
     * Get the value of the prompt.
     */
    public function value(): bool
    {}
}
