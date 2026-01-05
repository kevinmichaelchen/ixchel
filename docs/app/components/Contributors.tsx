'use client';

import React from 'react';
import { Tweet } from 'react-tweet';
import styles from './Contributors.module.css';

export interface Contributor {
  name: string;
  project: string;
  role: string;
  color: string;
  tweetId?: string;
}

interface ContributorsProps {
  contributors: Contributor[];
}

export function Contributors({ contributors }: ContributorsProps) {
  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <h2>Standing on Giants' Shoulders</h2>
        <p>The thinkers and builders who shaped Helix Tools</p>
      </div>

      <div className={styles.grid}>
        {contributors.map((contributor, idx) => (
          <div
            key={idx}
            className={styles.card}
            style={{
              '--accent-color': contributor.color,
              '--delay': `${idx * 0.08}s`,
            } as React.CSSProperties}
          >
            <div className={styles.cardInner}>
              <div className={styles.accent} />
              <h3>{contributor.name}</h3>
              <p className={styles.project}>{contributor.project}</p>
              <span className={styles.role}>{contributor.role}</span>

              {contributor.tweetId && (
                <div className={styles.tweetContainer}>
                  <Tweet id={contributor.tweetId} />
                </div>
              )}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
