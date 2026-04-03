import { useState } from 'react';
import { fetchRecommendedOffers, createStudent, type AggregatedOffer } from '@/lib/api';
import { applyForInternship } from '@/lib/api';
import OfferCard from '@/components/OfferCard';
import { UserCircle, UserPlus } from 'lucide-react';

export default function StudentDashboard() {
    const [studentId, setStudentId] = useState('');
    const [offers, setOffers] = useState<AggregatedOffer[]>([]);
    const [hasScanned, setHasScanned] = useState(false);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    // Toast State
    const [toast, setToast] = useState<{ message: string; type: 'success' | 'error' } | null>(null);
    const [applyingId, setApplyingId] = useState<string | null>(null);

    // Create student form
    const [showCreateForm, setShowCreateForm] = useState(false);
    const [firstname, setFirstname] = useState('');
    const [name, setName] = useState('');
    const [domain, setDomain] = useState('');
    const [creating, setCreating] = useState(false);

    const handleCreate = async (e: React.FormEvent) => {
        e.preventDefault();
        if (!firstname.trim() || !name.trim() || !domain.trim()) return;

        setCreating(true);
        try {
            const student = await createStudent({ firstname: firstname.trim(), name: name.trim(), domain: domain.trim() });
            setStudentId(student.id);
            setShowCreateForm(false);
            setFirstname('');
            setName('');
            setDomain('');
            setToast({ message: `Student created! ID: ${student.id}`, type: 'success' });
            setTimeout(() => setToast(null), 5000);
        } catch (err: any) {
            setToast({ message: err.message, type: 'error' });
            setTimeout(() => setToast(null), 5000);
        } finally {
            setCreating(false);
        }
    };

    const handleSearch = async (e: React.FormEvent) => {
        e.preventDefault();
        if (!studentId.trim()) return;

        setLoading(true);
        setError(null);
        setHasScanned(true);

        try {
            const data = await fetchRecommendedOffers(studentId.trim());
            setOffers(data);
        } catch (err: any) {
            setError(err.message);
            setOffers([]);
        } finally {
            setLoading(false);
        }
    };

    const handleApply = async (offerId: string) => {
        setApplyingId(offerId);
        try {
            const result = await applyForInternship(studentId, offerId);
            setToast({
                message: `Application ${result.status}! ${result.message}`,
                type: result.status === 'Approved' ? 'success' : 'error'
            });
        } catch (err: any) {
            setToast({ message: err.message, type: 'error' });
        } finally {
            setApplyingId(null);
            setTimeout(() => setToast(null), 5000);
        }
    };

    return (
        <div>
            <div className="glass-panel" style={{ padding: '2rem', maxWidth: '600px', margin: '0 auto 3rem auto', textAlign: 'center' }}>
                <UserCircle size={48} color="var(--primary)" style={{ marginBottom: '1rem' }} />
                <h2>Student Dashboard</h2>
                <p style={{ marginBottom: '1.5rem' }}>Enter your Student ID to see personalized internship recommendations based on your domain.</p>

                <form onSubmit={handleSearch} style={{ display: 'flex', gap: '1rem' }}>
                    <input
                        type="text"
                        className="input-field"
                        placeholder="e.g. 550e8400-e29b-41d4-a716-446655440000"
                        value={studentId}
                        onChange={(e) => setStudentId(e.target.value)}
                        style={{ flex: 1, marginBottom: 0 }}
                        autoFocus
                    />
                    <button type="submit" className="btn" disabled={loading || !studentId.trim()}>
                        {loading ? <div className="loader" /> : 'Get Recommendations'}
                    </button>
                </form>

                <div style={{ marginTop: '1rem' }}>
                    <button
                        type="button"
                        className="btn btn-secondary"
                        onClick={() => setShowCreateForm(!showCreateForm)}
                    >
                        <UserPlus size={18} />
                        {showCreateForm ? 'Cancel' : 'New Student'}
                    </button>
                </div>

                {showCreateForm && (
                    <form onSubmit={handleCreate} style={{ marginTop: '1.5rem', textAlign: 'left' }}>
                        <div className="input-group">
                            <label style={{ fontSize: '0.875rem', color: 'var(--text-muted)' }}>First Name</label>
                            <input
                                type="text"
                                className="input-field"
                                placeholder="Jean"
                                value={firstname}
                                onChange={(e) => setFirstname(e.target.value)}
                            />
                        </div>
                        <div className="input-group">
                            <label style={{ fontSize: '0.875rem', color: 'var(--text-muted)' }}>Last Name</label>
                            <input
                                type="text"
                                className="input-field"
                                placeholder="Dupont"
                                value={name}
                                onChange={(e) => setName(e.target.value)}
                            />
                        </div>
                        <div className="input-group">
                            <label style={{ fontSize: '0.875rem', color: 'var(--text-muted)' }}>Domain</label>
                            <input
                                type="text"
                                className="input-field"
                                placeholder="e.g. Computer Science"
                                value={domain}
                                onChange={(e) => setDomain(e.target.value)}
                            />
                        </div>
                        <button
                            type="submit"
                            className="btn btn-success"
                            disabled={creating || !firstname.trim() || !name.trim() || !domain.trim()}
                            style={{ width: '100%' }}
                        >
                            {creating ? <div className="loader" /> : 'Create Student'}
                        </button>
                    </form>
                )}
            </div>

            {error ? (
                <div style={{ color: 'var(--danger)', textAlign: 'center', padding: '2rem', background: 'rgba(239, 68, 68, 0.1)', borderRadius: 'var(--radius-md)' }}>
                    {error}
                </div>
            ) : hasScanned && !loading && offers.length === 0 ? (
                <div style={{ textAlign: 'center', padding: '4rem', color: 'var(--text-muted)', background: 'var(--bg-card)', borderRadius: 'var(--radius-lg)' }}>
                    No recommendations found. Try checking your domain or exploring all offers.
                </div>
            ) : offers.length > 0 ? (
                <div>
                    <h3 style={{ marginBottom: '1.5rem', fontSize: '1.25rem' }}>Recommended for you</h3>
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
                <div className={`toast ${toast.type}`}>
                    <span>{toast.message}</span>
                </div>
            )
            }
        </div >
    );
}
