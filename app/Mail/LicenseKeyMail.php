<?php

namespace App\Mail;

use Illuminate\Bus\Queueable;
use Illuminate\Mail\Mailable;
use Illuminate\Mail\Mailables\Content;
use Illuminate\Mail\Mailables\Envelope;
use Illuminate\Queue\SerializesModels;

class LicenseKeyMail extends Mailable
{
    use Queueable, SerializesModels;

    public function __construct(
        public string $licenseKey,
    ) {}

    public function envelope(): Envelope
    {
        return new Envelope(
            subject: 'Your ChromaVale Pro license key',
        );
    }

    public function content(): Content
    {
        return new Content(
            view: 'emails.license-key',
            with: ['licenseKey' => $this->licenseKey],
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
