
import { type AggregatedOffer } from '@/lib/api';
import CityScoreBars from '@/components/CityScoreBars';
import { Building2, MapPin, Briefcase, ExternalLink, Activity } from 'lucide-react';

interface OfferCardProps {
    data: AggregatedOffer;
    onApply?: (offerId: string) => void;
    isApplying?: boolean;
}

function calculateAverageScore(score: any) {
    if (!score) return null;
    return (score.quality_of_life + score.safety + score.economy + score.culture) / 4;
}

export default function OfferCard({ data, onApply, isApplying }: OfferCardProps) {
    const { offer, city_score, latest_news } = data;
    const avgScore = calculateAverageScore(city_score);

    return (
        <div className="glass-panel" style={{ padding: '1.5rem', display: 'flex', flexDirection: 'column', gap: '1rem', height: '100%' }}>

            {/* Header */}
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start' }}>
                <div>
                    <h3 style={{ fontSize: '1.25rem', fontWeight: 600, color: '#fff', marginBottom: '0.25rem' }}>
                        {offer.name}
                    </h3>
                    <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem', color: 'var(--text-muted)', fontSize: '0.875rem' }}>
                        <Building2 size={14} />
                        <span>{offer.company}</span>
                        <span style={{ color: 'var(--glass-border)' }}>•</span>
                        <MapPin size={14} />
                        <span>{offer.city}</span>
                    </div>
                </div>
                <span className="badge">{offer.domain}</span>
            </div>

            {/* City Scores */}
            {city_score ? (
                <div style={{ marginTop: '0.5rem', padding: '1rem', background: 'rgba(0,0,0,0.2)', borderRadius: 'var(--radius-md)' }}>
                    <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '0.75rem' }}>
                        <span style={{ fontSize: '0.875rem', fontWeight: 500, display: 'flex', alignItems: 'center', gap: '0.4rem' }}>
                            <Activity size={14} color="var(--secondary)" /> City Metrics
                        </span>
                        <span style={{ fontSize: '1rem', fontWeight: 700, color: 'var(--text-main)' }}>
                            {Math.round(avgScore!)} <span style={{ fontSize: '0.75rem', color: 'var(--text-muted)' }}>/ 2000</span>
                        </span>
                    </div>
                    <CityScoreBars score={city_score} />
                </div>
            ) : (
                <div style={{ marginTop: '0.5rem', padding: '1rem', background: 'rgba(0,0,0,0.2)', borderRadius: 'var(--radius-md)', textAlign: 'center', color: 'var(--text-muted)', fontSize: '0.875rem' }}>
                    No city metrics available
                </div>
            )}

            {/* Latest News Sneak Peek */}
            {latest_news && latest_news.length > 0 && (
                <div style={{ fontSize: '0.875rem', color: 'var(--text-muted)' }}>
                    <span style={{ fontWeight: 500, color: 'var(--text-main)' }}>Latest News: </span>
                    {latest_news[0].name}
                </div>
            )}

            {/* Actions */}
            <div style={{ marginTop: 'auto', display: 'flex', gap: '0.75rem', paddingTop: '1rem' }}>
                <a
                    href={offer.url}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="btn btn-secondary"
                    style={{ flex: 1, textDecoration: 'none' }}
                >
                    <ExternalLink size={16} /> Details
                </a>

                {onApply && (
                    <button
                        onClick={() => onApply(offer.id)}
                        disabled={isApplying}
                        className="btn"
                        style={{ flex: 1 }}
                    >
                        {isApplying ? <div className="loader"></div> : <><Briefcase size={16} /> Apply</>}
                    </button>
                )}
            </div>

        </div>
    );
}
