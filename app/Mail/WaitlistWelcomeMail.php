<?php

namespace App\Mail;

use Illuminate\Bus\Queueable;
use Illuminate\Mail\Mailable;
use Illuminate\Mail\Mailables\Content;
use Illuminate\Mail\Mailables\Envelope;
use Illuminate\Queue\SerializesModels;

class WaitlistWelcomeMail extends Mailable
{
    use Queueable, SerializesModels;

    /**
     * @param  'mac'|'windows'|null  $platform
     */
    public function __construct(
        public ?string $platform = null,
    ) {}

    public function envelope(): Envelope
    {
        return new Envelope(
            subject: 'You are on the ChromaVale waitlist',
        );
    }

    public function content(): Content
    {
        return new Content(
            view: 'emails.waitlist-welcome',
            with: [
                'platformLabel' => match ($this->platform) {
                    'mac' => 'macOS',
                    'windows' => 'Windows',
                    default => 'your device',
                },
            ],
        );
    }

    /**
     * @return array<int, \Illuminate\Mail\Mailables\Attachment>
     */
    public function attachments(): array
    {
        return [];
    }
}
