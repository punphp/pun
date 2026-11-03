<?php

namespace Laravel\Prompts\Concerns;

trait Colors
{
    /**
     * Reset all colors and styles.
     */
    public function reset(string $text): string
    {}

    /**
     * Make the text bold.
     */
    public function bold(string $text): string
    {}

    /**
     * Make the text dim.
     */
    public function dim(string $text): string
    {}

    /**
     * Make the text italic.
     */
    public function italic(string $text): string
    {}

    /**
     * Underline the text.
     */
    public function underline(string $text): string
    {}

    /**
     * Invert the text and background colors.
     */
    public function inverse(string $text): string
    {}

    /**
     * Hide the text.
     */
    public function hidden(string $text): string
    {}

    /**
     * Strike through the text.
     */
    public function strikethrough(string $text): string
    {}

    /**
     * Set the text color to black.
     */
    public function black(string $text): string
    {}

    /**
     * Set the text color to red.
     */
    public function red(string $text): string
    {}

    /**
     * Set the text color to green.
     */
    public function green(string $text): string
    {}

    /**
     * Set the text color to yellow.
     */
    public function yellow(string $text): string
    {}

    /**
     * Set the text color to blue.
     */
    public function blue(string $text): string
    {}

    /**
     * Set the text color to magenta.
     */
    public function magenta(string $text): string
    {}

    /**
     * Set the text color to cyan.
     */
    public function cyan(string $text): string
    {}

    /**
     * Set the text color to white.
     */
    public function white(string $text): string
    {}

    /**
     * Set the text background to black.
     */
    public function bgBlack(string $text): string
    {}

    /**
     * Set the text background to red.
     */
    public function bgRed(string $text): string
    {}

    /**
     * Set the text background to green.
     */
    public function bgGreen(string $text): string
    {}

    /**
     * Set the text background to yellow.
     */
    public function bgYellow(string $text): string
    {}

    /**
     * Set the text background to blue.
     */
    public function bgBlue(string $text): string
    {}

    /**
     * Set the text background to magenta.
     */
    public function bgMagenta(string $text): string
    {}

    /**
     * Set the text background to cyan.
     */
    public function bgCyan(string $text): string
    {}

    /**
     * Set the text background to white.
     */
    public function bgWhite(string $text): string
    {}

    /**
     * Set the text color to gray.
     */
    public function gray(string $text): string
    {}
}
