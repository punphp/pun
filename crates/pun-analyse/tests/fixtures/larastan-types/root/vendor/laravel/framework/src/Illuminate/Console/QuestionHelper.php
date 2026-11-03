<?php

namespace Illuminate\Console;

use Illuminate\Console\View\Components\TwoColumnDetail;
use Illuminate\Support\Stringable;
use Symfony\Component\Console\Formatter\OutputFormatter;
use Symfony\Component\Console\Helper\SymfonyQuestionHelper;
use Symfony\Component\Console\Output\OutputInterface;
use Symfony\Component\Console\Question\ChoiceQuestion;
use Symfony\Component\Console\Question\ConfirmationQuestion;
use Symfony\Component\Console\Question\Question;

class QuestionHelper extends SymfonyQuestionHelper
{
    /**
     * {@inheritdoc}
     *
     * @return void
     */
    #[\Override]
    protected function writePrompt(OutputInterface $output, Question $question): void
    {}

    /**
     * Ensures the given string ends with punctuation.
     *
     * @param  string  $string
     * @return string
     */
    protected function ensureEndsWithPunctuation($string)
    {}
}
