export interface Offer {
    id: string;
    name: string;
    company: string;
    city: string;
    domain: string;
    url: string;
}

export interface CityScore {
    city: string;
    country: string;
    updated_at: string;
    quality_of_life: number;
    safety: number;
    economy: number;
    culture: number;
}

export interface AggregatedOffer {
    offer: Offer;
    city_score: CityScore | null;
    latest_news: any[] | null;
}

const API_BASE = 'http://localhost:8080';

export async function fetchOffers(domain?: string, city?: string): Promise<AggregatedOffer[]> {
    const params = new URLSearchParams();
    if (domain) params.append('domain', domain);
    if (city) params.append('city', city);

    const url = `${API_BASE}/offer${params.toString() ? '?' + params.toString() : ''}`;

    const res = await fetch(url);
    if (!res.ok) throw new Error('Failed to fetch offers');
    return res.json();
}

export async function fetchRecommendedOffers(studentId: string): Promise<AggregatedOffer[]> {
    const res = await fetch(`${API_BASE}/student/${studentId}/recommended-offers`);
    if (!res.ok) throw new Error('Failed to fetch recommended offers');
    return res.json();
}

export async function applyForInternship(studentId: string, offerId: string): Promise<any> {
    const res = await fetch(`${API_BASE}/internship`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ student_id: studentId, offer_id: offerId })
    });

    const data = await res.json();
    if (!res.ok) throw new Error(data.error || 'Application failed');
    return data;
}
