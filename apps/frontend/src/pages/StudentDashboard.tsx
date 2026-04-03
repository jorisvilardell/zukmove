import { useState } from 'react';
import { fetchRecommendedOffers, fetchStudent, createStudent, applyForInternship, sortOffersByScore, type AggregatedOffer, type SortByScore } from '@/lib/api';
import OfferCard from '@/components/OfferCard';
import { UserCircle, UserPlus, LogIn, ArrowDownWideNarrow } from 'lucide-react';
import { useAuthStore } from '@/store/auth';

const SORT_OPTIONS: { value: SortByScore | ''; label: string }[] = [
    { value: '', label: 'Default' },
    { value: 'safety', label: 'Safety' },
    { value: 'economy', label: 'Economy' },
    { value: 'quality_of_life', label: 'Quality of Life' },
    { value: 'culture', label: 'Culture' },
];

export default function StudentDashboard() {
    const student = useAuthStore((s) => s.student);
    const login = useAuthStore((s) => s.login);

    const [loginId, setLoginId] = useState('');
    const [loginLoading, setLoginLoading] = useState(false);
    const [rawOffers, setRawOffers] = useState<AggregatedOffer[]>([]);
    const [sortBy, setSortBy] = useState<SortByScore | ''>('');
    const [hasScanned, setHasScanned] = useState(false);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const [toast, setToast] = useState<{ message: string; type: 'success' | 'error' } | null>(null);
    const [applyingId, setApplyingId] = useState<string | null>(null);

    // Create student form
    const [showCreateForm, setShowCreateForm] = useState(false);
    const [firstname, setFirstname] = useState('');
    const [name, setName] = useState('');
    const [domain, setDomain] = useState('');
    const [creating, setCreating] = useState(false);

    const offers = sortBy ? sortOffersByScore(rawOffers, sortBy) : rawOffers;

    const showToast = (message: string, type: 'success' | 'error') => {
        setToast({ message, type });
        setTimeout(() => setToast(null), 5000);
    };

    const handleLogin = async (e: React.FormEvent) => {
        e.preventDefault();
        if (!loginId.trim()) return;

        setLoginLoading(true);
        try {
            const studentData = await fetchStudent(loginId.trim());
            login(studentData);
            showToast(`Welcome back, ${studentData.firstname}!`, 'success');
        } catch {
            showToast('Student not found. Check your ID or create an account.', 'error');
        } finally {
            setLoginLoading(false);
        }
    };

    const handleCreate = async (e: React.FormEvent) => {
        e.preventDefault();
        if (!firstname.trim() || !name.trim() || !domain.trim()) return;

        setCreating(true);
        try {
            const created = await createStudent({ firstname: firstname.trim(), name: name.trim(), domain: domain.trim() });
            login(created);
            setShowCreateForm(false);
            setFirstname('');
            setName('');
            setDomain('');
            showToast(`Account created! Your ID: ${created.id}`, 'success');
        } catch (err: any) {
            showToast(err.message, 'error');
        } finally {
            setCreating(false);
        }
    };

    const handleLoadRecommendations = async () => {
        if (!student) return;

        setLoading(true);
        setError(null);
        setHasScanned(true);

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

    // --- Not logged in ---
    if (!student) {
        return (
            <div>
                <div className="glass-panel" style={{ padding: '2rem', maxWidth: '500px', margin: '0 auto', textAlign: 'center' }}>
                    <UserCircle size={48} color="var(--primary)" style={{ marginBottom: '1rem' }} />
                    <h2>Student Login</h2>
                    <p style={{ marginBottom: '1.5rem' }}>Enter your Student ID to log in, or create a new account.</p>

                    <form onSubmit={handleLogin} style={{ display: 'flex', gap: '1rem' }}>
                        <input
                            type="text"
                            className="input-field"
                            placeholder="Your Student ID (UUID)"
                            value={loginId}
                            onChange={(e) => setLoginId(e.target.value)}
                            style={{ flex: 1, marginBottom: 0 }}
                            autoFocus
                        />
                        <button type="submit" className="btn" disabled={!loginId.trim() || loginLoading}>
                            {loginLoading ? <div className="loader" /> : <><LogIn size={18} /> Login</>}
                        </button>
                    </form>

                    <div style={{ marginTop: '1.5rem', borderTop: '1px solid var(--glass-border)', paddingTop: '1.5rem' }}>
                        <button
                            type="button"
                            className="btn btn-secondary"
                            onClick={() => setShowCreateForm(!showCreateForm)}
                        >
                            <UserPlus size={18} />
                            {showCreateForm ? 'Cancel' : 'Create New Account'}
                        </button>
                    </div>

                    {showCreateForm && (
                        <form onSubmit={handleCreate} style={{ marginTop: '1.5rem', textAlign: 'left' }}>
                            <div className="input-group">
                                <label style={{ fontSize: '0.875rem', color: 'var(--text-muted)' }}>First Name</label>
                                <input type="text" className="input-field" placeholder="Jean" value={firstname} onChange={(e) => setFirstname(e.target.value)} />
                            </div>
                            <div className="input-group">
                                <label style={{ fontSize: '0.875rem', color: 'var(--text-muted)' }}>Last Name</label>
                                <input type="text" className="input-field" placeholder="Dupont" value={name} onChange={(e) => setName(e.target.value)} />
                            </div>
                            <div className="input-group">
                                <label style={{ fontSize: '0.875rem', color: 'var(--text-muted)' }}>Domain</label>
                                <input type="text" className="input-field" placeholder="e.g. Computer Science" value={domain} onChange={(e) => setDomain(e.target.value)} />
                            </div>
                            <button type="submit" className="btn btn-success" disabled={creating || !firstname.trim() || !name.trim() || !domain.trim()} style={{ width: '100%' }}>
                                {creating ? <div className="loader" /> : 'Create & Login'}
                            </button>
                        </form>
                    )}
                </div>

                {toast && (
                    <div className={`toast ${toast.type}`}><span>{toast.message}</span></div>
                )}
            </div>
        );
    }

    // --- Logged in ---
    return (
        <div>
            {/* Student Profile Card */}
            <div className="glass-panel" style={{ padding: '2rem', maxWidth: '600px', margin: '0 auto 2rem auto', textAlign: 'center' }}>
                <UserCircle size={48} color="var(--primary)" style={{ marginBottom: '1rem' }} />
                <h2 style={{ marginBottom: '0.25rem' }}>{student.firstname} {student.name}</h2>
                <p style={{ fontSize: '0.8rem', marginBottom: '0.75rem' }}>ID: <code style={{ fontSize: '0.75rem', background: 'rgba(0,0,0,0.3)', padding: '0.15rem 0.4rem', borderRadius: '4px' }}>{student.id}</code></p>
                {student.domain && <span className="badge">{student.domain}</span>}
            </div>

            {/* Recommendations Controls */}
            <div style={{ display: 'flex', gap: '1rem', alignItems: 'center', justifyContent: 'center', marginBottom: '2rem', flexWrap: 'wrap' }}>
                <button className="btn" onClick={handleLoadRecommendations} disabled={loading}>
                    {loading ? <div className="loader" /> : 'Get Recommendations'}
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
            </div>

            {/* Results */}
            {error ? (
                <div style={{ color: 'var(--danger)', textAlign: 'center', padding: '2rem', background: 'rgba(239, 68, 68, 0.1)', borderRadius: 'var(--radius-md)' }}>
                    {error}
                </div>
            ) : hasScanned && !loading && offers.length === 0 ? (
                <div style={{ textAlign: 'center', padding: '4rem', color: 'var(--text-muted)', background: 'var(--bg-card)', borderRadius: 'var(--radius-lg)' }}>
                    No recommendations found for your domain.
                </div>
            ) : offers.length > 0 ? (
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
            ) : null}

            {toast && (
                <div className={`toast ${toast.type}`}><span>{toast.message}</span></div>
            )}
        </div>
    );
}
