import { useState } from 'react';
import { useAuthStore } from '@/store/auth';
import { usePreferencesStore } from '@/store/preferences';
import { Navigate } from 'react-router-dom';
import { Mail, MessageCircle, Bell, BellOff, Trash2, AlertTriangle } from 'lucide-react';

export default function PreferencesPage() {
    const student = useAuthStore((s) => s.student);
    const { channel, contact, enabled, setChannel, setContact, setEnabled, reset } = usePreferencesStore();
    const [showConfirm, setShowConfirm] = useState(false);
    const [toast, setToast] = useState<{ message: string; type: 'success' | 'error' } | null>(null);

    if (!student) return <Navigate to="/login" replace />;

    const showToast = (message: string, type: 'success' | 'error') => {
        setToast({ message, type });
        setTimeout(() => setToast(null), 5000);
    };

    const handleSave = () => {
        if (!contact.trim()) {
            showToast('Please enter a contact address.', 'error');
            return;
        }
        showToast('Preferences saved successfully!', 'success');
    };

    const handleUnsubscribe = () => {
        reset();
        setShowConfirm(false);
        showToast('You have been unsubscribed. All preferences have been cleared.', 'success');
    };

    return (
        <div style={{ maxWidth: '600px', margin: '0 auto' }}>
            <h2 style={{ marginBottom: '1.5rem' }}>Notification Preferences</h2>

            {/* Channel Selection */}
            <div className="glass-panel" style={{ padding: '2rem', marginBottom: '1.5rem' }}>
                <h3 style={{ marginBottom: '1rem', fontSize: '1.1rem' }}>Notification Channel</h3>
                <p style={{ marginBottom: '1.5rem', fontSize: '0.875rem' }}>Choose how you want to receive alerts about new offers matching your domain.</p>

                <div style={{ display: 'flex', gap: '1rem', marginBottom: '1.5rem' }}>
                    <button
                        className={`channel-btn ${channel === 'email' ? 'active' : ''}`}
                        onClick={() => setChannel('email')}
                    >
                        <Mail size={24} />
                        <span>Email</span>
                    </button>
                    <button
                        className={`channel-btn ${channel === 'discord' ? 'active' : ''}`}
                        onClick={() => setChannel('discord')}
                    >
                        <MessageCircle size={24} />
                        <span>Discord</span>
                    </button>
                </div>

                <div className="input-group">
                    <label style={{ fontSize: '0.875rem', color: 'var(--text-muted)' }}>
                        {channel === 'email' ? 'Email Address' : 'Discord User ID'}
                    </label>
                    <input
                        type={channel === 'email' ? 'email' : 'text'}
                        className="input-field"
                        placeholder={channel === 'email' ? 'you@example.com' : 'username#1234'}
                        value={contact}
                        onChange={(e) => setContact(e.target.value)}
                    />
                </div>

                <button className="btn" onClick={handleSave} style={{ width: '100%' }}>
                    Save Preferences
                </button>
            </div>

            {/* Toggle */}
            <div className="glass-panel" style={{ padding: '2rem', marginBottom: '1.5rem' }}>
                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                    <div>
                        <h3 style={{ fontSize: '1.1rem', marginBottom: '0.25rem' }}>
                            {enabled ? 'Notifications Active' : 'Notifications Paused'}
                        </h3>
                        <p style={{ fontSize: '0.875rem' }}>
                            {enabled
                                ? 'You will receive alerts when new offers match your domain.'
                                : 'Notifications are paused. Toggle to resume.'}
                        </p>
                    </div>
                    <button
                        onClick={() => setEnabled(!enabled)}
                        className={`toggle-btn ${enabled ? 'active' : ''}`}
                        aria-label="Toggle notifications"
                    >
                        <div className="toggle-knob" />
                        {enabled ? <Bell size={14} /> : <BellOff size={14} />}
                    </button>
                </div>
            </div>

            {/* Danger Zone */}
            <div className="glass-panel danger-zone" style={{ padding: '2rem' }}>
                <h3 style={{ fontSize: '1.1rem', marginBottom: '0.5rem', color: 'var(--danger)', display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
                    <AlertTriangle size={18} /> Danger Zone
                </h3>
                <p style={{ fontSize: '0.875rem', marginBottom: '1rem' }}>
                    Permanently unsubscribe and clear all notification preferences. This action cannot be undone.
                </p>

                {!showConfirm ? (
                    <button
                        className="btn"
                        style={{ background: 'var(--danger)', width: '100%' }}
                        onClick={() => setShowConfirm(true)}
                    >
                        <Trash2 size={16} /> Unsubscribe & Delete Preferences
                    </button>
                ) : (
                    <div style={{ display: 'flex', flexDirection: 'column', gap: '0.75rem', padding: '1rem', background: 'rgba(239,68,68,0.1)', borderRadius: 'var(--radius-md)' }}>
                        <p style={{ fontSize: '0.875rem', fontWeight: 600, color: 'var(--danger)' }}>
                            Are you sure? This will erase all your notification settings.
                        </p>
                        <div style={{ display: 'flex', gap: '0.75rem' }}>
                            <button className="btn" style={{ background: 'var(--danger)', flex: 1 }} onClick={handleUnsubscribe}>
                                Yes, Unsubscribe
                            </button>
                            <button className="btn btn-secondary" style={{ flex: 1 }} onClick={() => setShowConfirm(false)}>
                                Cancel
                            </button>
                        </div>
                    </div>
                )}
            </div>

            {toast && (
                <div className={`toast ${toast.type}`}><span>{toast.message}</span></div>
            )}
        </div>
    );
}
