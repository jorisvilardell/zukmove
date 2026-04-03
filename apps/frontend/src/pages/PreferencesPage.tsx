import { useState, useEffect } from 'react';
import { useAuthStore } from '@/store/auth';
import { fetchSubscriber, updateSubscriber, deleteSubscriber, type Subscriber } from '@/lib/api';
import { Mail, MessageCircle, Bell, BellOff, Trash2, AlertTriangle, RefreshCw } from 'lucide-react';

export default function PreferencesPage() {
    const student = useAuthStore((s) => s.student);

    const [subscriber, setSubscriber] = useState<Subscriber | null>(null);
    const [loading, setLoading] = useState(true);
    const [notFound, setNotFound] = useState(false);

    const [channel, setChannel] = useState<'email' | 'discord'>('email');
    const [contact, setContact] = useState('');
    const [enabled, setEnabled] = useState(true);
    const [saving, setSaving] = useState(false);

    const [showConfirm, setShowConfirm] = useState(false);
    const [toast, setToast] = useState<{ message: string; type: 'success' | 'error' } | null>(null);

    useEffect(() => {
        if (student) loadSubscriber();
    }, [student?.id]);

    const loadSubscriber = async () => {
        if (!student) return;
        setLoading(true);
        setNotFound(false);
        try {
            const data = await fetchSubscriber(student.id);
            setSubscriber(data);
            setChannel(data.channel);
            setContact(data.contact);
            setEnabled(data.enabled);
        } catch {
            setNotFound(true);
            setSubscriber(null);
        } finally {
            setLoading(false);
        }
    };

    const showToast = (message: string, type: 'success' | 'error') => {
        setToast({ message, type });
        setTimeout(() => setToast(null), 5000);
    };

    const handleSave = async () => {
        if (!student || !contact.trim()) {
            showToast('Please enter a contact address.', 'error');
            return;
        }
        setSaving(true);
        try {
            const updated = await updateSubscriber(student.id, { channel, contact: contact.trim(), enabled });
            setSubscriber(updated);
            setNotFound(false);
            showToast('Preferences saved successfully!', 'success');
        } catch (err: any) {
            showToast(err.message, 'error');
        } finally {
            setSaving(false);
        }
    };

    const handleUnsubscribe = async () => {
        if (!student) return;
        try {
            await deleteSubscriber(student.id);
            setSubscriber(null);
            setNotFound(true);
            setChannel('email');
            setContact('');
            setEnabled(true);
            setShowConfirm(false);
            showToast('You have been unsubscribed.', 'success');
        } catch (err: any) {
            showToast(err.message, 'error');
        }
    };

    if (!student) {
        return (
            <div style={{ textAlign: 'center', padding: '4rem', color: 'var(--text-muted)' }}>
                Please log in from the Dashboard to access preferences.
            </div>
        );
    }

    if (loading) {
        return (
            <div style={{ display: 'flex', justifyContent: 'center', padding: '4rem' }}>
                <div className="loader" style={{ width: '40px', height: '40px', borderWidth: '4px' }} />
            </div>
        );
    }

    return (
        <div style={{ maxWidth: '600px', margin: '0 auto' }}>
            <h2 style={{ marginBottom: '1.5rem' }}>Notification Preferences</h2>

            {notFound && (
                <div className="glass-panel" style={{ padding: '1.5rem', marginBottom: '1.5rem', textAlign: 'center' }}>
                    <p style={{ fontSize: '0.875rem', marginBottom: '0.75rem' }}>
                        No subscription found. Configure your preferences below and save to subscribe.
                    </p>
                </div>
            )}

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

                <button className="btn" onClick={handleSave} disabled={saving} style={{ width: '100%' }}>
                    {saving ? <div className="loader" /> : <><RefreshCw size={16} /> Save Preferences</>}
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
            {subscriber && (
                <div className="glass-panel danger-zone" style={{ padding: '2rem' }}>
                    <h3 style={{ fontSize: '1.1rem', marginBottom: '0.5rem', color: 'var(--danger)', display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
                        <AlertTriangle size={18} /> Danger Zone
                    </h3>
                    <p style={{ fontSize: '0.875rem', marginBottom: '1rem' }}>
                        Permanently unsubscribe and remove all notification preferences from La Poste.
                    </p>

                    {!showConfirm ? (
                        <button
                            className="btn"
                            style={{ background: 'var(--danger)', width: '100%' }}
                            onClick={() => setShowConfirm(true)}
                        >
                            <Trash2 size={16} /> Unsubscribe
                        </button>
                    ) : (
                        <div style={{ display: 'flex', flexDirection: 'column', gap: '0.75rem', padding: '1rem', background: 'rgba(239,68,68,0.1)', borderRadius: 'var(--radius-md)' }}>
                            <p style={{ fontSize: '0.875rem', fontWeight: 600, color: 'var(--danger)' }}>
                                Are you sure? This will delete your subscription.
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
            )}

            {toast && (
                <div className={`toast ${toast.type}`}><span>{toast.message}</span></div>
            )}
        </div>
    );
}
