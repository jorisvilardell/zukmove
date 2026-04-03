import { useState, useEffect } from 'react';
import { fetchRecommendedOffers, fetchStudent, createStudent, applyForInternship, sortOffersByScore, fetchNotifications, markNotificationRead, type AggregatedOffer, type SortByScore, type Notification } from '@/lib/api';
import OfferCard from '@/components/OfferCard';
import { UserCircle, UserPlus, LogIn, ArrowDownWideNarrow, Bell, BellOff, Check } from 'lucide-react';
import { useAuthStore } from '@/store/auth';
import { Navigate } from 'react-router-dom';

const SORT_OPTIONS: { value: SortByScore | ''; label: string }[] = [
    { value: '', label: 'Default' },
    { value: 'safety', label: 'Safety' },
    { value: 'economy', label: 'Economy' },
    { value: 'quality_of_life', label: 'Quality of Life' },
    { value: 'culture', label: 'Culture' },
];

function OfferSkeleton() {
    return (
        <div className="glass-panel skeleton-card" style={{ padding: '1.5rem', display: 'flex', flexDirection: 'column', gap: '1rem' }}>
            <div style={{ display: 'flex', justifyContent: 'space-between' }}>
                <div style={{ flex: 1 }}>
                    <div className="skeleton" style={{ height: '1.25rem', width: '70%', marginBottom: '0.5rem' }} />
                    <div className="skeleton" style={{ height: '0.875rem', width: '40%' }} />
                </div>
                <div className="skeleton" style={{ height: '1.5rem', width: '4rem', borderRadius: '999px' }} />
            </div>
            <div style={{ padding: '1rem', background: 'rgba(0,0,0,0.2)', borderRadius: 'var(--radius-md)' }}>
                <div className="skeleton" style={{ height: '6px', width: '100%', marginBottom: '0.5rem' }} />
                <div className="skeleton" style={{ height: '6px', width: '100%', marginBottom: '0.5rem' }} />
                <div className="skeleton" style={{ height: '6px', width: '100%' }} />
            </div>
        </div>
    );
}

export default function StudentDashboard() {
    const student = useAuthStore((s) => s.student);
    const seenNewsIds = useAuthStore((s) => s.seenNewsIds);
    const markNewsSeen = useAuthStore((s) => s.markNewsSeen);

    const [rawOffers, setRawOffers] = useState<AggregatedOffer[]>([]);
    const [sortBy, setSortBy] = useState<SortByScore | ''>('');
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const [toast, setToast] = useState<{ message: string; type: 'success' | 'error' } | null>(null);
    const [applyingId, setApplyingId] = useState<string | null>(null);
    const [showNotifications, setShowNotifications] = useState(false);

    // Notifications
    const [notifications, setNotifications] = useState<Notification[]>([]);
    const [notifLoading, setNotifLoading] = useState(false);
    const [showNotifications, setShowNotifications] = useState(false);

    const offers = sortBy ? sortOffersByScore(rawOffers, sortBy) : rawOffers;
    const unreadCount = notifications.filter((n) => !n.read).length;

    useEffect(() => {
        if (student) {
            loadNotifications();
        }
    }, [student?.id]);

    const loadNotifications = async () => {
        if (!student) return;
        setNotifLoading(true);
        try {
            const data = await fetchNotifications(student.id);
            setNotifications(data);
        } catch {
            // Notifications service may not be available yet
        } finally {
            setNotifLoading(false);
        }
    };

    const handleMarkRead = async (notifId: string) => {
        try {
            await markNotificationRead(notifId);
            setNotifications((prev) =>
                prev.map((n) => (n.id === notifId ? { ...n, read: true } : n))
            );
        } catch {
            showToast('Failed to mark notification as read', 'error');
        }
    };

    // Collect all news as notifications
    const allNews: (News & { offerTitle: string })[] = rawOffers.flatMap((agg) =>
        (agg.latest_news ?? []).map((n) => ({ ...n, offerTitle: agg.offer.title }))
    );
    const unseenCount = allNews.filter((n) => !seenNewsIds.includes(n.id)).length;

    useEffect(() => {
        if (student) loadRecommendations();
    }, [student?.id]);

    const loadRecommendations = async () => {
        if (!student) return;
        setLoading(true);
        setError(null);
        try {
            const data = await fetchRecommendedOffers(student.id);
            setRawOffers(data);
        } catch (err: any) {
            setError(err.message);
            setRawOffers([]);
        } finally {
            setLoading(false);
        }
    };

    const showToast = (message: string, type: 'success' | 'error') => {
        setToast({ message, type });
        setTimeout(() => setToast(null), 5000);
    };

    const handleApply = async (offerId: string) => {
        if (!student) return;
        setApplyingId(offerId);
        try {
            const result = await applyForInternship(student.id, offerId);
            showToast(
                `Application ${result.status}! ${result.message}`,
                result.status === 'Approved' ? 'success' : 'error'
            );
        } catch (err: any) {
            showToast(err.message, 'error');
        } finally {
            setApplyingId(null);
        }
    };

    const handleMarkAllRead = () => {
        markNewsSeen(allNews.map((n) => n.id));
    };

    if (!student) return <Navigate to="/login" replace />;

    return (
        <div>
            {/* Student Profile Card */}
            <div className="glass-panel" style={{ padding: '2rem', maxWidth: '600px', margin: '0 auto 2rem auto', textAlign: 'center' }}>
                <UserCircle size={48} color="var(--primary)" style={{ marginBottom: '1rem' }} />
                <h2 style={{ marginBottom: '0.25rem' }}>{student.firstname} {student.name}</h2>
                <p style={{ fontSize: '0.8rem', marginBottom: '0.75rem' }}>
                    ID: <code style={{ fontSize: '0.75rem', background: 'rgba(0,0,0,0.3)', padding: '0.15rem 0.4rem', borderRadius: '4px' }}>{student.id}</code>
                </p>
                {student.domain && <span className="badge">{student.domain}</span>}
            </div>

            {/* Controls */}
            <div style={{ display: 'flex', gap: '1rem', alignItems: 'center', justifyContent: 'center', marginBottom: '2rem', flexWrap: 'wrap' }}>
                <button className="btn" onClick={loadRecommendations} disabled={loading}>
                    {loading ? <div className="loader" /> : 'Refresh Recommendations'}
                </button>

                {rawOffers.length > 0 && (
                    <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
                        <ArrowDownWideNarrow size={16} color="var(--text-muted)" />
                        <select
                            className="input-field"
                            value={sortBy}
                            onChange={(e) => setSortBy(e.target.value as SortByScore | '')}
                            style={{ width: 'auto', marginBottom: 0, padding: '0.5rem 0.75rem', fontSize: '0.875rem' }}
                        >
                            {SORT_OPTIONS.map((opt) => (
                                <option key={opt.value} value={opt.value}>{opt.label}</option>
                            ))}
                        </select>
                    </div>
                )}

                <button
                    className="btn btn-secondary"
                    onClick={() => { setShowNotifications(!showNotifications); if (!showNotifications) loadNotifications(); }}
                    style={{ position: 'relative' }}
                >
                    {showNotifications ? <BellOff size={18} /> : <Bell size={18} />}
                    Alerts
                    {unreadCount > 0 && (
                        <span className="notification-badge">{unreadCount}</span>
                    )}
                </button>
            </div>

            {/* Notification Panel */}
            {showNotifications && (
                <div className="glass-panel" style={{ padding: '1.5rem', marginBottom: '2rem', maxWidth: '700px', margin: '0 auto 2rem auto' }}>
                    <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '1rem' }}>
                        <h3 style={{ display: 'flex', alignItems: 'center', gap: '0.5rem', fontSize: '1.1rem' }}>
                            <Bell size={18} color="var(--primary)" /> Notifications
                            {unreadCount > 0 && <span className="notification-badge" style={{ position: 'static' }}>{unreadCount} new</span>}
                        </h3>
                        <button className="btn btn-secondary" onClick={loadNotifications} style={{ fontSize: '0.8rem', padding: '0.4rem 0.8rem' }}>
                            Refresh
                        </button>
                    </div>

                    {notifLoading ? (
                        <div style={{ display: 'flex', justifyContent: 'center', padding: '2rem' }}>
                            <div className="loader" />
                        </div>
                    ) : notifications.length === 0 ? (
                        <p style={{ textAlign: 'center', fontSize: '0.875rem', padding: '1rem' }}>No notifications yet.</p>
                    ) : (
                        <div style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem', maxHeight: '350px', overflowY: 'auto' }}>
                            {notifications.map((notif) => (
                                <div
                                    key={notif.id}
                                    style={{
                                        display: 'flex', justifyContent: 'space-between', alignItems: 'center',
                                        padding: '0.75rem 1rem',
                                        background: notif.read ? 'rgba(0,0,0,0.15)' : 'rgba(99,102,241,0.1)',
                                        borderRadius: 'var(--radius-sm)',
                                        borderLeft: notif.read ? '3px solid transparent' : '3px solid var(--primary)',
                                    }}
                                >
                                    <div style={{ flex: 1 }}>
                                        <div style={{ fontSize: '0.85rem', fontWeight: notif.read ? 400 : 600, color: 'var(--text-main)' }}>
                                            {notif.message}
                                        </div>
                                        <div style={{ fontSize: '0.7rem', color: 'var(--text-muted)', marginTop: '0.2rem' }}>
                                            {notif.type === 'new_offer' ? 'New Offer' : notif.type}
                                        </div>
                                    </div>
                                    {!notif.read && (
                                        <button
                                            onClick={() => handleMarkRead(notif.id)}
                                            className="btn btn-secondary"
                                            style={{ padding: '0.3rem 0.5rem', fontSize: '0.75rem', marginLeft: '0.75rem' }}
                                            title="Mark as read"
                                        >
                                            <Check size={14} />
                                        </button>
                                    )}
                                </div>
                            ))}
                        </div>
                    )}
                </div>
            )}

            {/* Results */}
            {loading ? (
                <div className="grid-cards">
                    {Array.from({ length: 4 }).map((_, i) => (
                        <OfferSkeleton key={i} />
                    ))}
                </div>
            ) : error ? (
                <div style={{ color: 'var(--danger)', textAlign: 'center', padding: '2rem', background: 'rgba(239, 68, 68, 0.1)', borderRadius: 'var(--radius-md)' }}>
                    {error}
                </div>
            ) : offers.length === 0 ? (
                <div style={{ textAlign: 'center', padding: '4rem', color: 'var(--text-muted)', background: 'var(--bg-card)', borderRadius: 'var(--radius-lg)' }}>
                    No recommendations found for your domain.
                </div>
            ) : (
                <div>
                    <h3 style={{ marginBottom: '1.5rem', fontSize: '1.25rem' }}>
                        Recommended for you
                        {sortBy && <span style={{ fontSize: '0.875rem', color: 'var(--text-muted)', fontWeight: 400 }}> — sorted by {SORT_OPTIONS.find(o => o.value === sortBy)?.label}</span>}
                    </h3>
                    <div className="grid-cards">
                        {offers.map((agg) => (
                            <OfferCard
                                key={agg.offer.id}
                                data={agg}
                                onApply={handleApply}
                                isApplying={applyingId === agg.offer.id}
                            />
                        ))}
                    </div>
                </div>
            )}

            {toast && (
                <div className={`toast ${toast.type}`}><span>{toast.message}</span></div>
            )}
        </div>
    );
}
