<?php

namespace UnsetNativeReflectionWrites;

/** @property string $p */
class WritableOverlay { public int|string $p = 1; }

/** @property-read string $p */
class ReadOverlay { public int|string $p = 1; }

/** @property-read string $p */
class AnnotationOnly {}

function writes(WritableOverlay $writable, ReadOverlay $read, AnnotationOnly $magic): void
{
    $writable->p = 1;
    $writable->p = 'allowed';
    $read->p = 'allowed';
    $magic->p = 1;
}
