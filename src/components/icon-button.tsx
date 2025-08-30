import clsx from 'clsx';
import { ButtonProps, ButtonSize, buttonVariants } from 'src/components/button';

interface IconButtonProps extends ButtonProps {
  tooltip?: string;
}

const sizeVariants: Record<ButtonSize, string> = {
  sm: 'p-2 text-sm',
  md: 'p-3 text-base',
  lg: 'p-4 text-lg',
};

export const IconButton = ({
  children,
  variant = 'primary',
  size = 'md',
  className,
  disabled,
  loading = false,
  tooltip,
  ...props
}: IconButtonProps) => {
  const iconButtonVariants = { ...buttonVariants, sizeVariants };
  return (
    <div className="relative group/tooltip">
      <button
        disabled={disabled || loading}
        className={clsx(
          'rounded-full focus:outline-none flex items-center justify-center cursor-pointer',
          iconButtonVariants.colorVariants[variant],
          iconButtonVariants.sizeVariants[size],
          (disabled || loading) && 'opacity-50 cursor-not-allowed',
          className
        )}
        data-tooltip-target="tooltip-dark"
        type="button"
        {...props}
      >
        {children}
      </button>
      {tooltip && (
        <div
          role="tooltip"
          className="absolute -left-8 px-2 py-1 text-xs text-white dark:bg-gray-800 rounded-md opacity-0 group-hover/tooltip:opacity-100 pointer-events-none whitespace-nowrap z-5 hidden group-hover/tooltip:inline-block"
        >
          {tooltip}
        </div>
      )}
    </div>
  );
};
