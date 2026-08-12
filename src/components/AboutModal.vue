<script setup lang="ts">
// About modal replicating the old_code about-content: logo, repo link,
// author, stack, social links and attribution (opens URLs via plugin-opener).
import { openUrl } from "@tauri-apps/plugin-opener";
import { APP_INFO } from "../constants";
import AppModal from "./AppModal.vue";
import { useUiStore } from "../stores/ui";

const ui = useUiStore();

function openExternal(url: string): void {
  void openUrl(url);
}
</script>

<template>
  <AppModal v-if="ui.showAbout" :title="`About ${APP_INFO.name}`" @close="ui.closeAbout">
    <div class="about-content">
      <div class="about-main">
        <img src="/welcome.svg" alt="Logo" class="about-logo" />
        <p class="about-name">
          {{ APP_INFO.name }}
          <span class="about-version">{{ APP_INFO.version }}</span>
        </p>

        <a class="repo-link" href="#" @click.prevent="openExternal(APP_INFO.social.githubRepo)">
          <i class="mdi mdi-github" />
          View source code on GitHub
        </a>
      </div>

      <div class="about-side">
        <p class="dev-info">
          Made with 💚 by <strong>{{ APP_INFO.author.name }}</strong><br />
          <span class="tech-stack">{{ APP_INFO.author.stack }}</span>
        </p>

        <div class="social-links">
          <a
            class="social-btn github"
            aria-label="GitHub"
            href="#"
            @click.prevent="openExternal(APP_INFO.social.githubProfile)"
          >
            <i class="mdi mdi-github" />
          </a>
          <a
            class="social-btn linkedin"
            aria-label="LinkedIn"
            href="#"
            @click.prevent="openExternal(APP_INFO.social.linkedin)"
          >
            <i class="mdi mdi-linkedin" />
          </a>
          <a
            class="social-btn email"
            aria-label="Email"
            href="#"
            @click.prevent="openExternal(APP_INFO.social.email)"
          >
            <i class="mdi mdi-email-outline" />
          </a>
        </div>

        <a
          class="attribution"
          href="#"
          @click.prevent="openExternal(APP_INFO.attribution.storyset.url)"
        >
          {{ APP_INFO.attribution.storyset.text }}
        </a>
      </div>
    </div>
  </AppModal>
</template>

<style scoped>
.about-content {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 3rem;
}

.about-main {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  flex-shrink: 0;
}

.about-side {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 1.5rem;
}

.about-logo {
  height: 80px;
  margin-bottom: 1rem;
}

.about-name {
  font-size: 1.2rem;
  font-weight: 700;
  margin: 0 0 0.2rem;
  color: rgb(var(--v-theme-on-surface));
}

.about-version {
  font-weight: 400;
  opacity: 0.7;
  margin-left: 0.25rem;
}

.tech-stack {
  font-size: 0.8rem;
  color: rgba(var(--v-theme-on-surface), var(--v-medium-emphasis-opacity));
  background: rgba(var(--v-theme-on-surface), var(--v-hover-opacity));
  padding: 4px 12px;
  border-radius: 12px;
  margin-top: 8px;
  display: inline-block;
}

.dev-info {
  margin: 0;
  font-size: 0.9rem;
  color: rgba(var(--v-theme-on-surface), var(--v-medium-emphasis-opacity));
}

.social-links {
  display: flex;
  justify-content: flex-start;
  gap: 1.5rem;
}

.social-btn {
  color: rgba(var(--v-theme-on-surface), var(--v-medium-emphasis-opacity));
  transition: all 0.2s cubic-bezier(0.175, 0.885, 0.32, 1.275);
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  padding: 0;
  border-radius: 50%;
  background: transparent;
  border: 1px solid transparent;
  cursor: pointer;
  font-size: 1.5rem;
  text-decoration: none;
}

.social-btn:hover {
  transform: scale(1.2);
  background: rgba(var(--v-theme-on-surface), var(--v-hover-opacity));
}

.social-btn.github:hover {
  color: rgb(var(--v-theme-on-surface));
}

.social-btn.linkedin:hover {
  color: #0077b5;
}

.social-btn.email:hover {
  color: #ea4335;
}

.repo-link {
  margin-top: 0.5rem;
  font-size: 0.9rem;
  color: rgb(var(--v-theme-primary));
  text-decoration: none;
  display: inline-flex;
  align-items: center;
  gap: 6px;
  cursor: pointer;
}

.repo-link:hover {
  text-decoration: underline;
}

.attribution {
  font-size: 0.7rem;
  color: rgba(var(--v-theme-on-surface), var(--v-medium-emphasis-opacity));
  text-decoration: none;
  opacity: 0.5;
  display: block;
  cursor: pointer;
}

.attribution:hover {
  opacity: 1;
  text-decoration: underline;
}
</style>
